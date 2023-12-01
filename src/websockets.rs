use axum::{
    body::Body,
    extract::{ws::WebSocket, WebSocketUpgrade},
    http::Response,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WsInputData {
    pub device_id: i128,

    #[serde(flatten)]
    pub inner: WsInnerData,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum WsInnerData {
    Move,
    Temp { temp: f64, hum: f64 },
}

pub async fn websocket_handler(ws: WebSocketUpgrade) -> Response<Body> {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            let data = msg.clone().into_data();
            let data: WsInputData = serde_json::from_slice(&data).unwrap();
            println!("{:?}", data);

            msg
        } else {
            // client disconnected
            return;
        };

        if socket.send(msg).await.is_err() {
            // client disconnected
            return;
        }
    }
}
