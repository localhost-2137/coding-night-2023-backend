use axum::http::StatusCode;
use axum::{Extension, Json, Router};
use axum::routing::post;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, SqlitePool};
use crate::utils::jwt::JWTAuth;

pub fn router() -> Router {
    Router::new()
        .route("/register", post(register_controller))
        .route("/login", post(login_controller))
        .layer(CookieManagerLayer::new())
}

#[derive(Serialize, Deserialize, Clone)]
struct LoginDto {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct RegisterDto {
    email: String,
    password: String,
    name: String,
    lastname: String,
}

async fn login_controller(
    Extension(pool): Extension<SqlitePool>,
    cookies: Cookies,
    Json(login_dto): Json<LoginDto>,
) -> Result<String, (StatusCode, &'static str)> {
    let jwt_string = login_service(&pool, login_dto).await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to login"))?;

    let cookie = Cookie::build(("JWT_AUTH", jwt_string.clone()))
        .secure(true)
        .expires(None)
        .build();
    
    cookies.add(cookie);
    Ok(jwt_string)
}

async fn login_service(conn: &SqlitePool, login_dto: LoginDto) -> anyhow::Result<String> {
    let hashed_passwd = "";

    let res = sqlx::query!(
        "SELECT email, user_id FROM user WHERE ? = password",
        hashed_passwd,
    )
        .fetch_one(conn).await?;

    let jwt_struct = JWTAuth { email: res.email, id: res.user_id as u32 };
    let token_str = crate::utils::jwt::serialize_jwt(jwt_struct)?;

    Ok(token_str)
}

async fn register_controller(
    Extension(pool): Extension<SqlitePool>,
    Json(register_dto): Json<RegisterDto>) -> Result<String, StatusCode>
{
    register_service(&pool, register_dto).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok("User created".to_string())
}

async fn register_service(conn: &SqlitePool, register_dto: RegisterDto) -> anyhow::Result<()> {
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