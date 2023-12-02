use axum::http::HeaderValue;
use axum::{routing::get, Extension, Router};
use http::header::{CONTENT_TYPE, COOKIE};
use http::Method;
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;

pub fn router(conn: SqlitePool) -> Router {
    Router::new()
        .nest_service("/room", crate::room::router(conn.clone()))
        .nest_service("/auth", crate::auth::router(conn.clone()))
        .nest_service("/schedule", crate::schedule::router(conn.clone()))
        .route(
            "/ws/:device_id",
            get(crate::esp_websockets::websocket_handler),
        )
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_credentials(true)
                .allow_headers([COOKIE, CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE]),
        )
        .layer(Extension(conn))
}
