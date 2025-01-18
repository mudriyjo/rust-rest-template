use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use super::aggregate::{Aggregate, ConfigAggregate};

#[async_trait]
pub trait Event: Serialize + for<'a> Deserialize<'a> + Send + Sync {
    fn aggregate_type() -> String;
    fn event_type(&self) -> String;
    fn aggregate_id(&self) -> Uuid;
    fn version(&self) -> i32;    
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ConfigEvent {
    ConfigCreated { id: Uuid, name: String, description: String, version: i32 },
    ConfigUpdated { id: Uuid, name: String, description: String, version: i32 },
    ConfigDeleted { id: Uuid, version: i32 }
}

impl Event for ConfigEvent {
    fn aggregate_type() -> String {
        ConfigAggregate::aggregate_type()
    }

    fn event_type(&self) -> String {
        match self {
            ConfigEvent::ConfigCreated { .. } => "ConfigCreated".to_string(),
            ConfigEvent::ConfigUpdated { .. } => "ConfigUpdated".to_string(),
            ConfigEvent::ConfigDeleted { .. } => "ConfigDeleted".to_string()
        }
    }

    fn aggregate_id(&self) -> Uuid {
        match self {
            ConfigEvent::ConfigCreated { id, .. } => *id,
            ConfigEvent::ConfigUpdated { id, .. } => *id,
            ConfigEvent::ConfigDeleted { id, .. } => *id
        }
    }

    fn version(&self) -> i32 {
        match self {
            ConfigEvent::ConfigCreated { version, .. } => *version,
            ConfigEvent::ConfigUpdated { version, .. } => *version,
            ConfigEvent::ConfigDeleted { version, .. } => *version
        }
    }
}