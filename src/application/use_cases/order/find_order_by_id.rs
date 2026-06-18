use std::sync::Arc;

use uuid::Uuid;

use crate::domain::{entities::order::Order, repository::order_repository::OrderRepository};

pub struct FindOrderByIdUseCase {
    repository: Arc<dyn OrderRepository>,
}

impl FindOrderByIdUseCase {
    pub fn new(repository: Arc<dyn OrderRepository>) -> Self {
        Self { repository }
    }
}

impl FindOrderByIdUseCase {
    pub async fn execute(&self, id: Uuid) -> anyhow::Result<Option<Order>> {
        let order = self.repository.find_by_id(id).await?;

        Ok(order)
    }
}
