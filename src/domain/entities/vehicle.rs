use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Vehicle {
    id: Uuid,
    plate: String,
    model: String,
    capacity_kg: f64,
    active: bool,
}
