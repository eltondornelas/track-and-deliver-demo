use std::fmt;

use chrono::{DateTime, SubsecRound, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Default)]
pub struct Order {
    pub id: Uuid,
    pub customer_name: String,
    pub status: OrderStatusEnum,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Default)]
pub enum OrderStatusEnum {
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
            updated_at: chrono::Utc::now().trunc_subsecs(3),
            ..Default::default()
        }
    }

    pub fn create(customer_name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            customer_name,
            status: OrderStatusEnum::Created,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl fmt::Display for OrderStatusEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            OrderStatusEnum::Created => "CREATED",
            OrderStatusEnum::Approved => "APPROVED",
            OrderStatusEnum::Shipped => "SHIPPED",
            OrderStatusEnum::InTransit => "IN_TRANSIT",
            OrderStatusEnum::Delivered => "DELIVERED",
            OrderStatusEnum::Cancelled => "CANCELLED",
        };

        write!(f, "{value}")
    }
}
