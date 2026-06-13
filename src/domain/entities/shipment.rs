use chrono::{DateTime, SubsecRound, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Default)]
pub struct Shipment {
    id: Uuid,
    order_id: Uuid,
    driver_id: Uuid,
    vehicle_id: Uuid,
    status: ShipmentStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Serialize, Default)]
enum ShipmentStatus {
    #[default]
    Pending,
    Assigned,
    InTransit,
    Delivered,
    Failed,
}

impl Shipment {
    pub fn new() -> Self {
        Shipment {
            created_at: chrono::Utc::now().trunc_subsecs(3),
            updated_at: chrono::Utc::now().trunc_subsecs(3),
            ..Default::default()
        }
    }
}
