use axum::Router;
use dotenvy::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Connection, SqliteConnection};
use std::sync::Arc;

mod auth;
mod utils;
mod middleware;
mod router;
mod websockets;

pub struct DbState {
    conn: SqliteConnection,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    _ = dotenv();

    let pool = SqlitePoolOptions::new()
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    let server = axum::serve(listener, router::router(pool));

    println!("Server is listening on port 3000");
    server
        .await
        .expect("Failed to start server on 0.0.0.0:3000");

    Ok(())
}
