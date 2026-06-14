-- Add migration script here
DROP TABLE IF EXISTS  society_security;
CREATE TABLE IF NOT EXISTS society_security
(
    society_id VARCHAR,
    security_id VARCHAR,
    CONSTRAINT fk_society FOREIGN KEY (society_id) REFERENCES society (id),
    CONSTRAINT fk_security_details FOREIGN KEY (security_id) REFERENCES  security_details (id)
);