use axum::{routing::get, Extension, Router};
use axum::http::HeaderValue;
use sqlx::SqlitePool;
use tower_http::cors::{Any, CorsLayer};

pub fn router(conn: SqlitePool) -> Router {
    Router::new()
        .nest_service("/room", crate::room::router(conn.clone()))
        .nest_service("/auth", crate::auth::router(conn.clone()))
        .nest_service("/schedule", crate::schedule::router(conn.clone()))
        .route("/ws", get(crate::esp_websockets::websocket_handler))
        .route(
            "/ws/:device_id",
            get(crate::esp_websockets::websocket_handler),
        )
        .layer(CorsLayer::new()
            .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
            .allow_headers(Any)
        )
        .layer(Extension(conn))
}
