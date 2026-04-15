use axum::Json;
use axum::{Router, routing::get};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct JsonResponse {
    message: String,
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(greet))
        .route("/other", get(other_route))
        .route("/msg", get(return_msg))
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
