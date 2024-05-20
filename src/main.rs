use axum::{response::Json, routing::get, Router};
use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::Read;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "4500".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/transactions", get(handler))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Json<Value> {
    let mut file = File::open("transactions.json").expect("transactions.json not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Failed to read file");

    let json_data: Value = serde_json::from_str(&data).expect("Failed to parse JSON");
    Json(json_data)
}
