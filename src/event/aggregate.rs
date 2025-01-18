use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use thiserror::Error;
use super::event::ConfigEvent;

#[async_trait]
pub trait Aggregate: Sized {
    type Event: Serialize + for<'a> Deserialize<'a> + Send + Sync;
    type Error;

    fn aggregate_type() -> String;
    fn aggregate_id(&self) -> Uuid;
    fn version(&self) -> i32;

    async fn apply_event(&mut self, event: Self::Event) -> Result<(), Self::Error>;
    async fn apply_events(&mut self, events: Vec<Self::Event>) -> Result<(), Self::Error> {
        for event in events {
            self.apply_event(event).await?;
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
enum AggregateError {
    #[error("Error applying event")]
    EventError
}

pub enum ConfigStatus {
    Active,
    Inactive
}

pub struct ConfigAggregate {
    id: Uuid,
    name: String,
    description: String,
    status: ConfigStatus,
    version: i32
}

impl Aggregate for ConfigAggregate {
    type Event = ConfigEvent;
    type Error = AggregateError;

    fn aggregate_type() -> String {
        "ConfigAggregate".to_string()
    }

    fn aggregate_id(&self) -> Uuid {
        self.id
    }

    fn version(&self) -> i32 {
        self.version
    }

    async fn apply_event(&mut self, event: Self::Event) -> Result<(), Self::Error> {
        match event {
            ConfigEvent::ConfigCreated { id, name, description, version } => {
                self.id = id;
                self.name = name;
                self.description = description;
                self.status = ConfigStatus::Active;
                self.version = version;
            },
            ConfigEvent::ConfigUpdated {id,  name, description, version } => {
                self.id = id;
                self.name = name;
                self.description = description;
                self.version = version;
            },
            ConfigEvent::ConfigDeleted { id, version } => {
                self.id = id;
                self.status = ConfigStatus::Inactive;
                self.version = version;
            }
        }

        Ok(())
    }
}