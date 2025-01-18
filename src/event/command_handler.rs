use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::event::Event;

#[async_trait]
pub trait Command: Sized + Serialize + for<'a> Deserialize<'a> {
    type Error;
}

#[async_trait]
pub trait CommandHandler<C : Command, E: Event> {
    async fn handle(&self, command: C) -> Result<E, C::Error>;
}