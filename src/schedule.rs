use axum::{Extension, Router};
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        .layer(Extension(pool))
        .layer(CorsLayer::permissive())
}


