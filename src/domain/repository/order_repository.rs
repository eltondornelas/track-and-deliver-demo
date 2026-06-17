use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::order::Order;

#[async_trait]
pub trait OrderRepository: Send + Sync {
    async fn save(&self, order: &Order) -> anyhow::Result<()>;
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Order>>;
}
