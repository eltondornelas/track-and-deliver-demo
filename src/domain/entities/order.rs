use chrono::{DateTime, SubsecRound, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct Order {
    id: String,
    customer_id: String, // UUID
    status: String,
    created_at: DateTime<Utc>,
}

impl Order {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Order {
    fn default() -> Self {
        Order {
            id: "1".to_string(),
            customer_id: "123abc321".to_string(),
            status: "CREATED".to_string(),
            created_at: chrono::Utc::now().trunc_subsecs(3),
        }
    }
}
