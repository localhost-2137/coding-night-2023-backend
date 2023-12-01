use axum::{Extension, Router};
use sqlx::SqlitePool;

pub fn router(conn: SqlitePool) -> Router {
    Router::new()
        .layer(Extension(conn))
        .nest_service("/auth", crate::auth::router())
}
