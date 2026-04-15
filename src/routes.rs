use axum::Json;
use axum::http::StatusCode;
use axum::{Router, routing::get};
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
