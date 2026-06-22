use serde::{Deserialize, Serialize};

use crate::domain::entities::order::Order;

#[derive(Deserialize, Serialize)]
pub struct OrderResponse {
    pub id: String,
    pub customer_name: String,
    pub status: String,
}

impl From<Order> for OrderResponse {
    fn from(order: Order) -> Self {
        Self {
            id: order.id.to_string(),
            customer_name: order.customer_name,
            status: order.status.to_string(),
        }
    }
}
