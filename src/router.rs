use axum::{routing::get, Extension, Router};
use sqlx::SqlitePool;

pub fn router(conn: SqlitePool) -> Router {
    Router::new()
        .layer(Extension(conn.clone()))
        .route("/ws", get(crate::websockets::websocket_handler))
        .nest_service("/room", crate::room::router(conn.clone()))
        .nest_service("/auth", crate::auth::router(conn))
}
