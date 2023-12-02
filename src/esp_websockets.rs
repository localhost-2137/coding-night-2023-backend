use axum::{
    body::Body,
    extract::{
        ws::{Message, WebSocket},
        Path, Request, State, WebSocketUpgrade,
    },
    http::Response,
    Extension,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc::UnboundedSender, Mutex};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WsInputData {
    pub device_id: i64,

    #[serde(flatten)]
    pub inner: WsInnerData,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum WsInnerData {
    Move,
    Temp { temp: f64, hum: f64, wh: f64 },
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum WsOutputData {
    Settings { presence_timeout: u64 },
}

lazy_static::lazy_static! {
    static ref CLIENTS: Arc<Mutex<HashMap<i64, UnboundedSender<WsOutputData>>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Path(device_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
) -> Response<Body> {
    return ws.on_upgrade(move |socket| async move {
        handle_socket(socket, device_id, pool).await;
    });
}

async fn handle_socket(mut socket: WebSocket, device_id: i64, pool: SqlitePool) {
    println!("New client: {}", device_id);
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    {
        _ = tx.send(WsOutputData::Settings {
            presence_timeout: 60000,
        });

        CLIENTS.lock().await.insert(device_id, tx);
    }

    loop {
        tokio::select! {
            Some(msg) = socket.recv() => {
                let msg = if let Ok(msg) = msg {
                    let data = msg.clone().into_data();
                    let data: WsInputData = if let Ok(data) = serde_json::from_slice(&data) {
                        data
                    } else {
                        continue;
                    };

                    if let WsInnerData::Temp { temp, hum, wh } = data.inner {
                        _ = sqlx::query!(
                            "INSERT INTO room_history (room_id, temperature, humidity, watthour, created_at)
                            VALUES (?, ?, ?, ?, datetime('now'))",
                            device_id,
                            temp,
                            hum,
                            wh,
                        )
                            .execute(&pool).await;
                    }

                    msg
                } else {
                    break;
                };

                if socket.send(msg).await.is_err() {
                    break;
                }
            }
            Some(msg) = rx.recv() => {
                let msg = serde_json::to_string(&msg).unwrap();
                let msg = Message::from(msg);
                if socket.send(msg).await.is_err() {
                    break;
                }
            }
            else => {
                println!("Client {} disconnected", device_id);
                break;
            }
        }
    }

    {
        CLIENTS.lock().await.remove(&device_id);
    }
}
