use crate::utils::jwt::JWTAuth;
use axum::extract::{Path, Query};
use axum::http::{HeaderValue, StatusCode};
use axum::routing::{get, patch, post};
use axum::{Extension, Json, Router};
use http::header::{CONTENT_TYPE, COOKIE};
use http::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{Executor, SqlitePool};
use tower_http::cors::{Any, CorsLayer};

#[derive(Serialize, Deserialize, Clone)]
struct CreateRoomDto {
    device_id: i64,
    icon_id: u32,
    name: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct UpdateRoomDto {
    id: u32,
    name: Option<String>,
    device_id: Option<u64>,
    icon_id: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone)]
struct GetRoom {
    id: u32,
}

#[derive(Serialize, Deserialize, Clone)]
struct Room {
    id: u32,
    name: String,
    icon_id: u32,
    temperature: f64,
    humidity: f64,
    watthour: f64,
}

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/", get(get_room_controller))
        .route("/", post(create_room_controller))
        .route("/", patch(update_room_controller))
        .route("/:id", patch(update_room_controller))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_credentials(true)
                .allow_headers([COOKIE, CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE]),
        )
        .layer(Extension(pool))
}

async fn update_room_controller(
    Extension(pool): Extension<SqlitePool>,
    jwt_auth: JWTAuth,
    Json(update_room_dto): Json<UpdateRoomDto>,
) -> Result<String, (StatusCode, String)> {
    update_room_service(pool, jwt_auth.id, update_room_dto)
        .await
        .map_err(|e| {
            let err = e.to_string();
            (StatusCode::INTERNAL_SERVER_ERROR, err)
        })?;

    Ok("Successfully updated".to_string())
}

async fn update_room_service(
    pool: SqlitePool,
    user_id: u32,
    update_dto: UpdateRoomDto,
) -> anyhow::Result<()> {
    let query = sqlx::query!(
        r#"
        UPDATE room
            SET room_name = COALESCE(?, room_name),
            icon_id = COALESCE(?, icon_id)
            WHERE owner_id = ? AND room_id = ?
        "#,
        update_dto.name,
        update_dto.icon_id,
        user_id,
        update_dto.id
    );
    pool.execute(query).await?;
    Ok(())
}

async fn delete_room_controller(
    Extension(pool): Extension<SqlitePool>,
    jwt_auth: JWTAuth,
    Path(id): Path<u32>,
) -> Result<String, (StatusCode, String)> {
    delete_room_service(pool, jwt_auth.id, id)
        .await
        .map_err(|e| {
            let err = e.to_string();
            (StatusCode::INTERNAL_SERVER_ERROR, err)
        })?;

    Ok("Successfully updated".to_string())
}

async fn delete_room_service(pool: SqlitePool, user_id: u32, room_id: u32) -> anyhow::Result<()> {
    let query = sqlx::query!(
        r#"
        DELETE FROM room
            WHERE owner_id = ? AND room_id = ?
        "#,
        user_id,
        room_id
    );
    pool.execute(query).await?;
    Ok(())
}

async fn create_room_controller(
    Extension(pool): Extension<SqlitePool>,
    jwt_auth: JWTAuth,
    Json(create_room_dto): Json<CreateRoomDto>,
) -> Result<Json<Room>, (StatusCode, String)> {
    let res = create_room_service(pool, jwt_auth.id, create_room_dto)
        .await
        .map_err(|e| {
            let err = e.to_string();
            (StatusCode::INTERNAL_SERVER_ERROR, err)
        })?;

    Ok(Json(res))
}

async fn create_room_service(
    pool: SqlitePool,
    user_id: u32,
    create_room: CreateRoomDto,
) -> anyhow::Result<Room> {
    let res = sqlx::query!(
        "INSERT INTO room(room_id, owner_id, room_name, icon_id) VALUES (?, ?, ?, ?) RETURNING *",
        create_room.device_id,
        user_id,
        create_room.name,
        create_room.icon_id,
    )
    .fetch_one(&pool)
    .await?;

    Ok(Room {
        id: res.room_id as u32,
        name: res.room_name,
        icon_id: res.icon_id as u32,
        temperature: res.current_temperature,
        humidity: res.current_humidity,
        watthour: res.current_watthour,
    })
}

async fn get_room_controller(
    Extension(pool): Extension<SqlitePool>,
    get_room_query: Option<Query<GetRoom>>,
    jwt_auth: JWTAuth,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    if let Some(Query(GetRoom { id })) = get_room_query {
        let room = get_room_service(pool, id, jwt_auth.id).await.map_err(|e| {
            let err = e.to_string();
            (StatusCode::INTERNAL_SERVER_ERROR, err)
        })?;

        Ok(Json(json! {room}))
    } else {
        let rooms = get_all_rooms_service(pool, jwt_auth.id)
            .await
            .map_err(|e| {
                let err = e.to_string();
                (StatusCode::INTERNAL_SERVER_ERROR, err)
            })?;

        Ok(Json(json! {rooms}))
    }
}

async fn get_room_service(pool: SqlitePool, room_id: u32, user_id: u32) -> anyhow::Result<Room> {
    let res = sqlx::query!(
        "SELECT * FROM room WHERE room_id = ? AND owner_id=?",
        room_id,
        user_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Room {
        id: res.room_id as u32,
        name: res.room_name,
        icon_id: res.icon_id as u32,
        temperature: res.current_temperature,
        humidity: res.current_humidity,
        watthour: res.current_watthour,
    })
}

async fn get_all_rooms_service(pool: SqlitePool, user_id: u32) -> anyhow::Result<Vec<Room>> {
    let rows = sqlx::query!("SELECT * FROM room WHERE owner_id=?", user_id)
        .fetch_all(&pool)
        .await?;

    let mut res = vec![];

    for row in rows {
        res.push(Room {
            id: row.room_id as u32,
            name: row.room_name,
            icon_id: row.icon_id as u32,
            temperature: row.current_temperature,
            humidity: row.current_humidity,
            watthour: row.current_watthour,
        });
    }

    Ok(res)
}
