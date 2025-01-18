use async_trait::async_trait;
use sqlx::types::Uuid;

use super::event::Event;

#[async_trait]
pub trait EventStore {
    type Error;

    async fn save_events<E : Event>(&self, events: Vec<E>) -> Result<(), Self::Error>;
    async fn get_events<E : Event>(&self, aggregate_id: Uuid) -> Result<Vec<E>, Self::Error>;
}