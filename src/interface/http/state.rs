use crate::application::use_cases::order::create_order::CreateOrderUseCase;
use std::sync::Arc;

pub struct AppState {
    pub create_order_use_case: Arc<CreateOrderUseCase>,
}
