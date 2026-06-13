-- Add migration script here
CREATE TABLE drivers (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    license_number VARCHAR(50) UNIQUE NOT NULL,
    phone VARCHAR(20),
    active BOOLEAN NOT NULL,
    created_at TIMESTAMP NOT NULL
);