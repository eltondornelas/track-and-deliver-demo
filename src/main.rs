use std::sync::Arc;

use crate::{
    application::use_cases::order::create_order::CreateOrderUseCase,
    infrastructure::{
        database::postgres::setup_database,
        repository::postgres::order_repository::PostgresOrderRepository,
    },
};

use axum::{Json, Router, routing::get};
use domain::entities::order::Order;

pub mod application;
pub mod domain;
pub mod infrastructure;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let pool = setup_database().await?;

    let app = Router::new().route("/order", get(get_order));

    let order_repository = Arc::new(PostgresOrderRepository::new(pool));

    let create_order_use_case = CreateOrderUseCase::new(order_repository);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn get_order() -> Json<Order> {
    axum::Json(Order::new())
}
