mod crypto;
mod routes;
mod state;

use axum::Router;
use state::AppState;
use std::sync::Arc;
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let (tx, _) = broadcast::channel::<String>(100);
    let state = Arc::new(AppState { tx });

    let app = Router::new()
        .nest("/api", routes::router(state))
        .layer(cors);

    let port = 3000;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    println!("Server running: http://127.0.0.1:{}", port);
    axum::serve(listener, app).await.unwrap();
}
