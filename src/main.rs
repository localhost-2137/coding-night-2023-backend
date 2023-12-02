use dotenvy::dotenv;
use sqlx::sqlite::SqlitePoolOptions;

mod auth;
mod esp_websockets;
mod middleware;
mod room;
mod router;
mod utils;
mod schedule;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    _ = dotenv();

    let pool = SqlitePoolOptions::new()
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    let server = axum::serve(listener, router::router(pool));

    println!("Server is listening on port 3000!");
    server
        .await
        .expect("Failed to start server on 0.0.0.0:3000");

    Ok(())
}
