mod config;
mod controller;
mod event;

use std::error::Error;
use axum::{response::{IntoResponse, Response}, routing::get, routing::post, Extension, Json};
use sea_orm::{Database};
use migration::{Migrator, MigratorTrait};
use config::config::get_config;
use serde::{Deserialize, Serialize};
use tracing::Level;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(
            hello_world,
        ),
        components(
            schemas(HelloWorldResponse)
        ),
        tags(
            (name = "hello", description = "Hello world endpoints")
        ))]
struct ApiDoc;

#[derive(ToSchema, Serialize, Deserialize)]
struct HelloWorldResponse {
    message: String,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "hello",
    responses(
        (status = 200, description = "Hello world message", body = HelloWorldResponse)
    )
)]
async fn hello_world() -> Json<HelloWorldResponse> {
    Json::from(HelloWorldResponse{
        message: "Hello world!".to_string()
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install().expect("Error with starting color eyre hook...");

    dotenvy::dotenv()?;

    let config = get_config();
    let pool = Database::connect(&config.database_url).await?;
    
    let openapi = ApiDoc::openapi();

    Migrator::up(&pool, None).await?;

    let connection = tokio::net::TcpListener::bind(&config.server_address).await?;

    let router = axum::Router::new()
        .route("/", get(hello_world))
        .route("/config", post(controller::controller::create_config))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
        .layer(Extension(pool.clone()));

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    tracing::info!("Start server...");
    tracing::info!("Config server address: {:?}, database: {:?}", &config.server_address, &config.database_url);
    axum::serve(connection, router).await?;

    Ok(())
}