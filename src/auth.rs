use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use axum::http::StatusCode;
use axum::{Extension, Json, Router};
use axum::routing::post;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, SqlitePool};
use crate::utils::jwt::JWTAuth;

pub fn router(conn: SqlitePool) -> Router {
    Router::new()
        .route("/register", post(register_controller))
        .route("/login", post(login_controller))
        .layer(CookieManagerLayer::new())
        .layer(Extension(conn))
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
) -> Result<String, (StatusCode, String)> {
    let jwt_string = login_service(&pool, login_dto).await
        .map_err(|e| {
            let err = e.to_string();
            (StatusCode::INTERNAL_SERVER_ERROR, err)
        })?;

    let cookie = Cookie::build(("JWT_AUTH", jwt_string.clone()))
        .secure(true)
        .expires(None)
        .build();

    cookies.add(cookie);
    Ok(jwt_string)
}

async fn login_service(conn: &SqlitePool, login_dto: LoginDto) -> anyhow::Result<String> {
    let hashed_password = hash_password(&login_dto.password)?;

    let res = sqlx::query!(
        "SELECT email, user_id, password FROM user WHERE email = ?",
        login_dto.email,
    )
        .fetch_one(conn).await?;
    
    let db_passwd_hash = PasswordHash::new(&res.password).map_err(|_| {
        anyhow::Error::msg("Failed to hash password")
    })?;
    Argon2::default().verify_password(login_dto.password.as_bytes(), &db_passwd_hash).map_err(|_| {
        anyhow::Error::msg("Failed to verify password")
    })?;

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
    let hashed_password = hash_password(&register_dto.password)?;

    let query = sqlx::query!(
        "INSERT INTO user(email, password, name, lastname) VALUES (?, ?, ?, ?)",
        register_dto.email,
        hashed_password,
        register_dto.name,
        register_dto.lastname,
    );

    conn.execute(query).await?;
    Ok(())
}

fn hash_password(passwd: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hashed_passwd = argon2.hash_password(passwd.as_bytes(), &salt).map_err(|_| anyhow::Error::msg("Failed to hash password"))?;

    Ok(hashed_passwd.to_string())
}