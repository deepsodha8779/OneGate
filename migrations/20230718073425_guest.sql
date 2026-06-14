-- Add migration script here
-- DROP TABLE IF EXISTS  guest;
CREATE TABLE IF NOT EXISTS guest
(
    id VARCHAR PRIMARY KEY,
    user_id VARCHAR,
    society_id VARCHAR,
    flat_name VARCHAR,
    CONSTRAINT fk_society FOREIGN KEY (society_id) REFERENCES society (id),
    CONSTRAINT fk_users FOREIGN KEY (user_id) REFERENCES users (id)

);