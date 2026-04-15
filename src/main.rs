mod crypto;
mod routes;

use axum::{Router, routing::get, routing::post};

#[tokio::main]
async fn main() {
    let app = Router::new().nest("/api", routes::router());
    let port = 3000;

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    println!("Server running: http://127.0.0.1:{}", port);
    axum::serve(listener, app).await.unwrap();
}
