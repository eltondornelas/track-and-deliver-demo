use chrono::{DateTime, SubsecRound, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Default)]
pub struct Driver {
    id: Uuid,
    name: String,
    license_number: String,
    phone: String,
    active: bool,
    created_at: DateTime<Utc>,
}

impl Driver {
    pub fn new() -> Self {
        Driver {
            created_at: chrono::Utc::now().trunc_subsecs(3),
            ..Default::default()
        }
    }
}
