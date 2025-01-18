use async_trait::async_trait;
use sea_orm::*;
use serde_json::json;
use sqlx::types::Uuid;
use thiserror::Error;
use super::{entity::{self, es_events}, event::Event, event_store::EventStore};

pub struct  PostgresEventStore {
    db: DatabaseConnection
}

impl PostgresEventStore {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[derive(Error, Debug)]
pub enum EventStoreError {
    #[error("Error inserting/updating aggregate")]
    AggregateInsertError,
    #[error("Error inserting event")]
    EventInsertError,
    #[error("Error create/commit transaction")]
    TrxError,
    #[error("Error make query to database")]
    DbError,
    #[error("Error converting event to JSON")]
    EventJsonConversionError,
}

#[async_trait]
impl EventStore for PostgresEventStore {
    type Error = EventStoreError;

    async fn save_events<E : Event>(&self, events: Vec<E>) -> Result<(), Self::Error> {
        let db = &self.db;
        
        // Start transaction
        let txn = db.begin().await.map_err(|_| EventStoreError::TrxError)?;

        for event in events {
            // Convert event to JSON
            let json_data = serde_json::to_value(&event).map_err(|_| EventStoreError::EventJsonConversionError)?;
            let meta_data = json!({
                "timestamp": chrono::Utc::now(),
                "event_type": event.event_type()
            });

            // Insert or update aggregate
            let aggregate_id = event.aggregate_id();
            let aggregate_version = event.version();

            let existing_aggregate = entity::es_aggregates::Entity::find_by_id(aggregate_id)
                .one(&txn)
                .await
                .map_err(|_| EventStoreError::DbError)?;

            if let Some(aggregate) = existing_aggregate {
                let mut active_model: entity::es_aggregates::ActiveModel = aggregate.into();
                active_model.version = Set(aggregate_version);
                active_model.update(&txn).await.map_err(|_| EventStoreError::AggregateInsertError)?;
            } else {
                let new_aggregate = entity::es_aggregates::ActiveModel {
                    id: Set(aggregate_id),
                    version: Set(aggregate_version),
                    aggregate_type: Set(E::aggregate_type()),
                    ..Default::default()
                };
                new_aggregate.insert(&txn)
                    .await
                    .map_err(|_| EventStoreError::AggregateInsertError)?;
            }

            // Insert event
            let event_model = entity::es_events::ActiveModel {
                aggregate_id: Set(aggregate_id),
                version: Set(aggregate_version),
                event_type: Set(event.event_type()),
                json_data: Set(json_data),
                meta_data: Set(meta_data),
                create_at: Set(chrono::Utc::now().naive_local()),
                ..Default::default()
            };

            event_model.insert(&txn)
                .await
                .map_err(|_| EventStoreError::EventInsertError)?;
        }

        // Commit transaction
        txn.commit()
            .await
            .map_err(|_| EventStoreError::TrxError)?;
        Ok(())
    }

    async fn get_events<E : Event>(&self, aggregate_id: Uuid) -> Result<Vec<E>, Self::Error> {
        es_events::Entity::find()
            .filter(es_events::Column::AggregateId.eq(aggregate_id))
            .all(&self.db)
            .await
            .map_err(|_| EventStoreError::DbError)?
            .into_iter()
            .map(|event| {
                let json_data = event.json_data;
                serde_json::from_value(json_data)
                    .map_err(|_| EventStoreError::EventJsonConversionError)
            })
            .collect()
    }
}