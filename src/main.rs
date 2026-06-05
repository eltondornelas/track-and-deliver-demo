use axum::{Json, Router, routing::get};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
struct Order {
    id: String,
    customer_id: String, // UUID
    status: String,
    created_at: DateTime<Utc>,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/order", get(get_order));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap()
}

async fn get_order() -> Json<Order> {
    axum::Json(Order {
        id: "1".to_string(),
        customer_id: "123abc321".to_string(),
        status: "CREATED".to_string(),
        created_at: chrono::Utc::now(),
    })
}
