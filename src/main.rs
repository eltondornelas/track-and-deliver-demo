use axum::{Json, Router, routing::get};
use domain::entities::order::Order;

pub mod domain;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/order", get(get_order));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap()
}

async fn get_order() -> Json<Order> {
    axum::Json(Order::new())
}
