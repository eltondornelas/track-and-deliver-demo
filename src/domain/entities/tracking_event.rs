use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct TrackingEvent {
    id: Uuid,
    shipment_id: Uuid,
    latitude: f64,
    longitude: f64,
    event_time: DateTime<Utc>,
}
