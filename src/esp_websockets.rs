use axum::{
    body::Body,
    extract::{
        ws::{Message, WebSocket},
        Request, WebSocketUpgrade,
    },
    http::Response,
};
use serde::{Deserialize, Serialize};
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

pub async fn websocket_handler(ws: WebSocketUpgrade, req: Request) -> Response<Body> {
    if let Some(device_id) = req.uri().query() {
        let device_id = device_id.parse::<i64>().unwrap();

        return ws.on_upgrade(move |socket| async move {
            handle_socket(socket, device_id).await;
        });
    } else {
        Response::builder()
            .status(400)
            .body(Body::empty())
            .expect("failed to render response")
    }
}

async fn handle_socket(mut socket: WebSocket, device_id: i64) {
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
                    let data: WsInputData = serde_json::from_slice(&data).unwrap();
                    println!("{:?}", data);

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
