use chrono::{DateTime, SubsecRound, Utc};
use serde::Serialize;

#[derive(Serialize, Default)]
pub struct Order {
    id: String,
    customer_id: String, // UUID
    status: OrderStatusEnum,
    created_at: DateTime<Utc>,
}

#[derive(Serialize, Default)]
enum OrderStatusEnum {
    #[default]
    Created,
    Approved,
    Shipped,
    InTransit,
    Delivered,
    Cancelled,
}

impl Order {
    pub fn new() -> Self {
        Order {
            created_at: chrono::Utc::now().trunc_subsecs(3),
            ..Default::default()
        }
    }
}
