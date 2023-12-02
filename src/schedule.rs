use axum::{Extension, Router};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        .layer(Extension(pool))
        .layer(CorsLayer::permissive())
}

#[derive(Serialize, Deserialize, Clone)]
struct NewScheduleDto {
    room_id: u32,
    repeat_on: String,
    on_from_temperature: f64,
    off_from_temperature: f64,
}

fn new_schedule_controller() {
    
}

fn new_schedule_service(conn: SqlitePool, dto: NewScheduleDto) -> anyhow::Result<()> {
    
    
    todo!()
}
