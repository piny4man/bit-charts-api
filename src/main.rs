use axum::{response::Json, routing::get, Router};
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/transactions", get(handler))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4500").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Json<Value> {
    let mut file = File::open("transactions.json").expect("transactions.json not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Failed to read file");

    let json_data: Value = serde_json::from_str(&data).expect("Failed to parse JSON");
    Json(json_data)
}
