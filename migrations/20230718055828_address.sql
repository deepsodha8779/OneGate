-- Add migration script here
CREATE TABLE IF NOT EXISTS  address
(
    id VARCHAR PRIMARY KEY,
    pin_code VARCHAR,
    city VARCHAR,
    state  VARCHAR,
    country VARCHAR,
    address_line1 VARCHAR,
    address_line2 VARCHAR
);

