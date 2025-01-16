use std::error::Error;
use sea_orm::Database;
use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    tracing_subscriber::fmt::init();

    let server_port_address = std::env::var("SERVER")?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = Database::connect(db_url).await?;
    
    
    Migrator::up(&pool, None).await?;

    Ok(())
}