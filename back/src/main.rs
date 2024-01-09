use std::sync::Arc;

use axum::routing::{get, post};
use socketioxide::SocketIo;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::FmtSubscriber;
mod handlers;
mod models;
mod modules;
mod util;
pub struct AppState {
    db: Pool<Postgres>,
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;
    
    info!("🐘 Connected to database");

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", modules::ws::on_connect);
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/register", post(handlers::register::register_handler))
        .route("/login", post(handlers::login::login_handler))
        .with_state(Arc::new(AppState { db: pool }))
        .layer(layer)
        .layer(cors);

    info!("🚀 Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
