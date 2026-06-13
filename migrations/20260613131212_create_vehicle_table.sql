-- Add migration script here
CREATE TABLE vehicles (
    id UUID PRIMARY KEY,
    plate VARCHAR(20) UNIQUE NOT NULL,
    model VARCHAR(100) NOT NULL,
    capacity_kg DOUBLE PRECISION NOT NULL,
    active BOOLEAN NOT NULL
);