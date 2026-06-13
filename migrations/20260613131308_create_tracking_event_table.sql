-- Add migration script here
CREATE TABLE tracking_events (
    id UUID PRIMARY KEY,
    shipment_id UUID NOT NULL,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    event_time TIMESTAMP NOT NULL,

    FOREIGN KEY (shipment_id)
        REFERENCES shipments(id)
);