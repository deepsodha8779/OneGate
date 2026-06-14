-- Add migration script here
DROP TABLE IF EXISTS  amenity;
CREATE TABLE IF NOT EXISTS amenity
(
    id VARCHAR PRIMARY KEY,
    name VARCHAR,
    description  VARCHAR,
    start_time VARCHAR,
    end_time  VARCHAR,
    society_id VARCHAR,
    CONSTRAINT fk_society FOREIGN KEY (society_id) REFERENCES society (id)
);