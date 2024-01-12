use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
};
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
    let app_state = Arc::new(AppState { db: pool });
    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/user/register", post(handlers::register::register_handler))
        .route("/user/login", post(handlers::login::login_handler))
        .route("/chat/create", post(handlers::chat_create::create_handler) .route_layer(middleware::from_fn_with_state(app_state.clone(), util::jwt::auth)),)
        .route( "/me", get(handlers::me_handler::get_me_handler) .route_layer(middleware::from_fn_with_state(app_state.clone(), util::jwt::auth)),)
        .with_state(app_state)
        .layer(layer)
        .layer(cors);

    info!("🚀 Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
