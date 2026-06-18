use std::sync::Arc;

use anyhow::Result;

use crate::{
    application::use_cases::order::command::CreateOrderCommand,
    domain::{entities::order::Order, repository::order_repository::OrderRepository},
};

pub struct CreateOrderUseCase {
    repository: Arc<dyn OrderRepository>,
}

impl CreateOrderUseCase {
    pub fn new(repository: Arc<dyn OrderRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, command: CreateOrderCommand) -> Result<Order> {
        let order = Order::create(command.customer_name);
        self.repository.save(&order).await?;

        Ok(order)
    }
}
