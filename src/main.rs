mod crypto;
mod routes;

use axum::{Router, routing::get, routing::post};

use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // CORS

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new().nest("/api", routes::router()).layer(cors);
    let port = 3000;

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    println!("Server running: http://127.0.0.1:{}", port);
    axum::serve(listener, app).await.unwrap();
}
