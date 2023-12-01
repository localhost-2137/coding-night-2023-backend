use axum::{Extension, Json, Router};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{SqlitePool};
use crate::utils::jwt::JWTAuth;

#[derive(Serialize, Deserialize, Clone)]
struct CreateRoomDto {
    device_id: i64,
    icon_id: u32,
    name: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct GetRoom {
    id: u32,
}

#[derive(Serialize, Deserialize, Clone)]
struct Room {
    id: u32,
    name: String,
    device_id: u64,
    icon_id: u32,
}

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/", get(get_room_controller))
        .route("/", post(create_room_controller))
        .layer(Extension(pool))
}

async fn create_room_controller(
    Extension(pool): Extension<SqlitePool>,
    jwt_auth: JWTAuth,
    Json(create_room_dto): Json<CreateRoomDto>,
) -> Result<Json<Room>, (StatusCode, String)> {
    let res = create_room_service(pool, jwt_auth.id, create_room_dto).await.map_err(|e| {
        let err = e.to_string();
        (StatusCode::INTERNAL_SERVER_ERROR, err)
    })?;
    
    Ok(Json(res))
}

async fn create_room_service(pool: SqlitePool, user_id: u32, create_room: CreateRoomDto) -> anyhow::Result<Room> {
    let res = sqlx::query!(
        "INSERT INTO room(device_id, owner_id, room_name, icon_id) VALUES (?, ?, ?, ?) RETURNING *",
        create_room.device_id,
        user_id,
        create_room.name,
        create_room.icon_id,
    ).fetch_one(&pool).await?;
    
    Ok(Room {
        id: res.room_id as u32,
        name: res.room_name,
        device_id: res.device_id as u64,
        icon_id: res.icon_id as u32,
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
        let rooms = get_all_rooms_service(pool, jwt_auth.id).await.map_err(|e| {
            let err = e.to_string();
            (StatusCode::INTERNAL_SERVER_ERROR, err)
        })?;

        Ok(Json(json! {rooms}))
    }
}

async fn get_room_service(pool: SqlitePool, room_id: u32, user_id: u32) -> anyhow::Result<Room> {
    let res = sqlx::query!("SELECT * FROM room WHERE room_id = ? AND owner_id=?", room_id, user_id)
        .fetch_one(&pool).await?;

    Ok(Room {
        id: res.room_id as u32,
        name: res.room_name,
        device_id: res.device_id as u64,
        icon_id: res.icon_id as u32,
    })
}

async fn get_all_rooms_service(pool: SqlitePool, user_id: u32) -> anyhow::Result<Vec<Room>> {
    let rows = sqlx::query!("SELECT * FROM room WHERE owner_id=?", user_id)
        .fetch_all(&pool).await?;

    let mut res = vec![];

    for row in rows {
        res.push(
            Room {
                id: row.room_id as u32,
                name: row.room_name,
                device_id: row.device_id as u64,
                icon_id: row.icon_id as u32,
            }
        );
    }

    Ok(res)
}