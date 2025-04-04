use tokio::net::TcpListener;
use axum::{Router, routing::{get, post}};
use tower_http::cors::{Any, CorsLayer};

mod handlers;
mod models;
mod img_ops;
mod utils;

use handlers::{process_image, test_process};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    
    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // Setup routes
    let app = Router::new()
        .route("/", get(|| async { "hello"}))
        .route("/process", post(process_image))
        .route("/postprocess", post(test_process))
        .layer(cors);

    println!("Server running on http://0.0.0.0:5000");
    let listener = TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}