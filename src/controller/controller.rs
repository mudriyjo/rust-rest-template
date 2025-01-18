use axum::{Extension, Json};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum CreateConfig {
    Succes {id: String },
    Error {error_msg: String }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload")]
pub enum ConfigCommand {
    CreateConfig { description: String, name: String }
}

pub async fn create_config(Extension(db): Extension<DatabaseConnection>, Json(cmd): Json<ConfigCommand>) -> Json<CreateConfig> {
    println!("{:?}", cmd);
    Json::from(CreateConfig::Succes{id: "xxxx-xxx-xxx-xxxx".to_string()})
}