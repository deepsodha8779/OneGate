-- Add migration script here
-- Add migration script here
-- Add migration script here
DROP TABLE IF EXISTS  auth;
CREATE TABLE IF NOT EXISTS auth
(
    id VARCHAR PRIMARY KEY,
    user_id VARCHAR,
    otp VARCHAR,
    otp_sent TIMESTAMP,
    attempts INT
);



