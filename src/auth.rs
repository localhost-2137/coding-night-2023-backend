use axum::http::StatusCode;
use axum::{Extension, Json, Router};
use axum::routing::post;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, SqlitePool};

pub fn router() -> Router {
    Router::new()
        .route("/register", post(register_controller))
}

#[derive(Serialize, Deserialize, Clone)]
struct UserRegisterDto {
    email: String,
    password: String,
    name: String,
    lastname: String,
}

async fn register_controller(
    Extension(pool): Extension<SqlitePool>,
    Json(register_dto): Json<UserRegisterDto>) -> Result<String, StatusCode>
{
    register_service(&pool, register_dto).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok("User created".to_string())
}

async fn register_service(conn: &SqlitePool, register_dto: UserRegisterDto) -> anyhow::Result<()> {
    let query = sqlx::query!(
        "INSERT INTO user(email, password, name, lastname) VALUES (?, ?, ?, ?)",
        register_dto.email,
        register_dto.password,
        register_dto.name,
        register_dto.lastname,
    );

    conn.execute(query).await?;
    Ok(())
}