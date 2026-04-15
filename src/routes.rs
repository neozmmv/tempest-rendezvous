use axum::Json;
use axum::http::StatusCode;
use axum::{Router, routing::get};
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::crypto::Crypto;
use crate::crypto::{AEScrypt, AESdecrypt};

#[derive(Serialize, Deserialize)]
struct JsonResponse {
    message: String,
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(greet))
        .route("/other", get(other_route))
        .route("/msg", get(return_msg))
        .route("/post", axum::routing::post(post))
        .route("/ws", get(ws_handler))
}

// websocket route for TEST

pub async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    // this is called on the router and just upgrades the connection to a websocket connection
    ws.on_upgrade(handle_socket)
}

// ws function itself
async fn handle_socket(mut socket: WebSocket) {
    // ws loop
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            // matches the message type
            Message::Text(text) => {
                // use serde_json to parse the message to a JsonResponse struct
                match serde_json::from_str::<JsonResponse>(&text) {
                    Ok(json_msg) => {
                        println!("Received JSON: {:?}", json_msg.message);
                        socket
                            .send(Message::Text(json_msg.message.into()))
                            .await
                            .unwrap();
                    }
                    Err(_) => {
                        println!("Received non-JSON text: {}", text);
                        socket
                            .send(Message::Text("Invalid JSON".into()))
                            .await
                            .unwrap();
                    }
                }
                //println!("received: {}", text);
                //socket.send(Message::Text(text)).await.unwrap(); // echo
            }
            Message::Close(_) => break,
            _ => {}
        }
    }
}

pub async fn post(Json(body): Json<JsonResponse>) -> (StatusCode, Json<Value>) {
    // returns if message is empty
    if body.message.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                   "error": "Message cannot be empty"
            })),
        );
    }

    let crypto = AEScrypt(&body.message);
    let decrypted = AESdecrypt(&crypto);
    // this is for testing the encryption functions
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
