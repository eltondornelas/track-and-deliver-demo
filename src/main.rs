use crate::infrastructure::database::postgres::setup_database;
use axum::{Json, Router, routing::get};
use domain::entities::order::Order;

pub mod domain;
pub mod infrastructure;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let pool = setup_database().await?;

    let app = Router::new().route("/order", get(get_order));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn get_order() -> Json<Order> {
    axum::Json(Order::new())
}
