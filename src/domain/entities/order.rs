use std::fmt;

use chrono::{DateTime, SubsecRound, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::infrastructure::repository::postgres_order_repository::OrderRow;

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

impl OrderStatusEnum {
    pub fn from(value: &str) -> anyhow::Result<Self> {
        match value {
            "CREATED" => Ok(Self::Created),
            "APPROVED" => Ok(Self::Approved),
            "SHIPPED" => Ok(Self::Shipped),
            "DELIVERED" => Ok(Self::Delivered),
            "CANCELLED" => Ok(Self::Cancelled),

            _ => anyhow::bail!("Invalid order status: {}", value),
        }
    }
}

impl TryFrom<OrderRow> for Order {
    type Error = anyhow::Error;

    fn try_from(row: OrderRow) -> Result<Self, Self::Error> {
        Ok(Order {
            id: row.id,
            customer_name: row.customer_name,
            status: OrderStatusEnum::from(row.status.as_str())?,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}
