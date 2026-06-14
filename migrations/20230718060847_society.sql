-- Add migration script here

-- Add migration script here
-- DROP TABLE IF EXISTS  society;
CREATE TABLE IF NOT EXISTS society
(
    id VARCHAR PRIMARY KEY,
    name VARCHAR,
    address_id VARCHAR,
    created_by  VARCHAR,
    created_at VARCHAR,
    allowed_attempts INT,
    maintenance_per_month INT,
    CONSTRAINT fk_address FOREIGN KEY (address_id) REFERENCES address (id)
);



