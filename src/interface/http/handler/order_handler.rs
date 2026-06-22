use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};

use crate::{
    application::use_cases::order::{
        command::CreateOrderCommand, create_order::CreateOrderUseCase,
    },
    interface::http::{
        request::create_order_request::CreateOrderRequest, response::order_response::OrderResponse,
        state::AppState,
    },
};

pub struct OrderHandler {
    create_order_use_case: Arc<CreateOrderUseCase>,
}

impl OrderHandler {
    pub fn new(create_order_use_case: Arc<CreateOrderUseCase>) -> Self {
        Self {
            create_order_use_case,
        }
    }
}

#[axum::debug_handler]
pub async fn create_order(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateOrderRequest>,
) -> Result<Json<OrderResponse>, StatusCode> {
    let command = CreateOrderCommand {
        customer_name: request.customer_name,
    };

    let order = state
        .create_order_use_case
        .execute(command)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(OrderResponse::from(order)))
}
