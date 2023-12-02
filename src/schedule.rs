use axum::{Extension, Json, Router};
use axum::routing::{get, post};
use http::header::{CONTENT_TYPE, COOKIE};
use http::{HeaderValue, StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;
use crate::utils::jwt::JWTAuth;

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/", get(get_all_schedules_controller))
        .route("/", post(new_schedule_controller))
        .layer(Extension(pool))
        .layer(CorsLayer::new()
            .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
            .allow_credentials(true)
            .allow_headers([COOKIE, CONTENT_TYPE])
        )
}

#[derive(Serialize, Deserialize, Clone)]
struct NewScheduleDto {
    room_id: u32,
    repeat_on: Option<String>,
    on_from_temperature: f64,
    off_from_temperature: f64,
    repeat_once: bool,
    trigger_after_ms: u64,
}

#[derive(Serialize, Deserialize, Clone)]
struct Schedule {
    id: u32,
    room_id: u64,
    repeat_once: bool,
    repeat_on: Option<String>,
    on_from_temperature: Option<f64>,
    off_from_temperature: Option<f64>,
    trigger_after_ms: Option<u64>,
}

async fn new_schedule_controller(
    Extension(pool): Extension<SqlitePool>,
    jwt_auth: JWTAuth,
    Json(new_schedule_dto): Json<NewScheduleDto>,
) -> Result<Json<Schedule>, (StatusCode, String)> {
    let res = new_schedule_service(pool, jwt_auth.id, new_schedule_dto).await.map_err(|e| {
        let err = e.to_string();
        (StatusCode::INTERNAL_SERVER_ERROR, err)
    })?;

    Ok(Json(res))
}

async fn new_schedule_service(pool: SqlitePool, user_id: u32, dto: NewScheduleDto) -> anyhow::Result<Schedule> {
    let trigger_after_ms = dto.trigger_after_ms as i32;

    let res = sqlx::query!(r#"
        INSERT INTO schedule(owner_id, room_id, repeat_on, on_from_temperature, off_from_temperature, repeat_once, trigger_after_ms)
        VALUES (?, ?, ?, ?, ?, ?, ?) RETURNING *
    "#,
        user_id,
        dto.room_id,
        dto.repeat_on,
        dto.on_from_temperature,
        dto.off_from_temperature,
        dto.repeat_once,
        trigger_after_ms,
    ).fetch_one(&pool).await?;

    Ok(Schedule {
        id: res.schedule_id as u32,
        room_id: res.room_id as u64,
        repeat_on: res.repeat_on,
        on_from_temperature: res.on_from_temperature,
        off_from_temperature: res.off_from_temperature,
        repeat_once: res.repeat_once,
        trigger_after_ms: res.trigger_after_ms.map(|e| e as u64),
    })
}

async fn get_all_schedules_controller(
    Extension(pool): Extension<SqlitePool>,
    jwt_auth: JWTAuth,
) -> Result<Json<Vec<Schedule>>, (StatusCode, String)> {
    let schedules = get_all_schedules_service(pool, jwt_auth.id).await.map_err(|e| {
        let err = e.to_string();
        (StatusCode::INTERNAL_SERVER_ERROR, err)
    })?;
    
    Ok(Json(schedules))
}

async fn get_all_schedules_service(pool: SqlitePool, user_id: u32) -> anyhow::Result<Vec<Schedule>> {
    let rows = sqlx::query!(r#"
        SELECT * FROM schedule WHERE owner_id = ?
    "#,user_id)
        .fetch_all(&pool).await?;

    let mut result = vec![];

    for row in rows {
        result.push(Schedule {
            id: row.schedule_id as u32,
            room_id: row.room_id as u64,
            repeat_once: row.repeat_once,
            repeat_on: row.repeat_on,
            on_from_temperature: row.on_from_temperature,
            off_from_temperature: row.off_from_temperature,
            trigger_after_ms: row.trigger_after_ms.map(|e| e as u64),
        });
    }

    Ok(result)
}
