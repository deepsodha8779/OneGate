-- Add migration script here
DROP TABLE IF EXISTS  society_guest;
CREATE TABLE IF NOT EXISTS society_guest
(
    society_id VARCHAR,
    guest_id VARCHAR,
    CONSTRAINT fk_society FOREIGN KEY (society_id) REFERENCES society (id),
    CONSTRAINT fk_guest FOREIGN KEY (guest_id) REFERENCES guest (id)
);