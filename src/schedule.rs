use axum::{Extension, Router};
use sqlx::SqlitePool;

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        .layer(Extension(pool))
}


