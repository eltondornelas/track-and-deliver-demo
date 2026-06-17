use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::entities::order::{Order, OrderStatusEnum};

#[derive(sqlx::FromRow)]
pub struct OrderRow {
    pub id: Uuid,
    pub customer_name: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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

impl From<&Order> for OrderRow {
    fn from(order: &Order) -> Self {
        Self {
            id: order.id,
            customer_name: order.customer_name.clone(),
            status: order.status.to_string(),
            created_at: order.created_at,
            updated_at: order.updated_at,
        }
    }
}
