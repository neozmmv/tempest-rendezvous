use crate::crypto::{AEScrypt, AESdecrypt};
use crate::state::AppState;
use axum::Json;
use axum::http::StatusCode;
use axum::{Router, routing::get};
use axum::{
    extract::State,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Serialize, Deserialize)]
struct JsonResponse {
    message: String,
}

pub fn router(state: Arc<AppState>) -> Router<()> {
    Router::new()
        .route("/", get(greet))
        .route("/other", get(other_route))
        .route("/msg", get(return_msg))
        .route("/post", axum::routing::post(post))
        .route("/ws", get(ws_handler))
        .with_state(state)
}

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    let tx = state.tx.clone();
    let mut rx = tx.subscribe();

    loop {
        tokio::select! {
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<JsonResponse>(&text) {
                            Ok(json_msg) => {
                                println!("Broadcasting: {:?}", json_msg.message);
                                let _ = tx.send(json_msg.message);
                            }
                            Err(_) => {
                                let _ = socket
                                    .send(Message::Text("Invalid JSON".into()))
                                    .await;
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }
            Ok(broadcast_msg) = rx.recv() => {
                if socket
                    .send(Message::Text(broadcast_msg.into()))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        }
    }
}

pub async fn post(Json(body): Json<JsonResponse>) -> (StatusCode, Json<Value>) {
    if body.message.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Message cannot be empty" })),
        );
    }
    let crypto = AEScrypt(&body.message);
    let decrypted = AESdecrypt(&crypto);
    (
        StatusCode::OK,
        Json(json!({
            "received": body.message,
            "encrypted": crypto.base64_encode(),
            "key": crypto.get_key_base64(),
            "decrypted": decrypted,
        })),
    )
}

pub async fn return_msg() -> Json<JsonResponse> {
    Json(JsonResponse {
        message: "Hello from the API!".to_string(),
    })
}

pub async fn greet() -> &'static str {
    "Greetings!"
}
pub async fn other_route() -> &'static str {
    "This is another route."
}
