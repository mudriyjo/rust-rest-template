mod config;
mod entity;

use std::error::Error;
use entity::prelude::EsEvents;
use sea_orm::{Database, EntityTrait};
use migration::{Migrator, MigratorTrait};
use config::config::get_config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    tracing_subscriber::fmt::init();

    let config = get_config();
    let pool = Database::connect(config.database_url).await?;
    
    Migrator::up(&pool, None).await?;
    let events = EsEvents::find().all(&pool).await?;

    Ok(())
}