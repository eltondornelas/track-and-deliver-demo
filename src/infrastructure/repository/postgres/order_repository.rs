use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::{entities::order::Order, repository::order_repository::OrderRepository},
    infrastructure::repository::postgres::models::order_row::OrderRow,
};

pub struct PostgresOrderRepository {
    pool: PgPool,
}

impl PostgresOrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OrderRepository for PostgresOrderRepository {
    async fn save(&self, order: &Order) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO orders
            (
                id,
                customer_name,
                status,
                created_at,
                updated_at
            )
            VALUES ($1,$2,$3,$4,$5)
            "#,
        )
        .bind(order.id)
        .bind(&order.customer_name)
        .bind(order.status.to_string())
        .bind(order.created_at)
        .bind(order.updated_at)
        .execute(&self.pool)
        .await?;
        // TODO: what if we want to return the entity saved?

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Order>> {
        let row = sqlx::query_as::<_, OrderRow>(
            r#"
            SELECT
                id,
                customer_name,
                status,
                created_at,
                updated_at
            FROM orders
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(Order::try_from(row)?)),
            None => Ok(None),
        }
    }
}
