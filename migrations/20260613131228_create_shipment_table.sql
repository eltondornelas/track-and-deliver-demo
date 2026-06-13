-- Add migration script here
CREATE TABLE shipments (
    id UUID PRIMARY KEY,
    order_id UUID NOT NULL,
    driver_id UUID NOT NULL,
    vehicle_id UUID NOT NULL,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,

    CONSTRAINT fk_order
        FOREIGN KEY(order_id)
        REFERENCES orders(id),

    CONSTRAINT fk_driver
        FOREIGN KEY(driver_id)
        REFERENCES drivers(id),

    CONSTRAINT fk_vehicle
        FOREIGN KEY(vehicle_id)
        REFERENCES vehicles(id)
);