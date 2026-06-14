-- Add migration script here
-- DROP TABLE IF EXISTS   contact_type;
-- CREATE TYPE contact_type AS ENUM ('mobilenumber', 'email');
DROP TABLE IF EXISTS contact;
CREATE TABLE IF NOT EXISTS contact
(
    id VARCHAR PRIMARY KEY,
    user_id VARCHAR,
    contact_type   contact_types,
    value VARCHAR,
    CONSTRAINT fk_users FOREIGN KEY (user_id) REFERENCES  users (id)
);

