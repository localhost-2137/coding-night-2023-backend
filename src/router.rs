use axum::{routing::get, Extension, Router};
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;

pub fn router(conn: SqlitePool) -> Router {
    Router::new()
        .layer(Extension(conn.clone()))
        .nest_service("/room", crate::room::router(conn.clone()))
        .nest_service("/auth", crate::auth::router(conn.clone()))
        .nest_service("/schedule", crate::schedule::router(conn))
        .route("/ws", get(crate::esp_websockets::websocket_handler))
        .layer(CorsLayer::permissive())
}
