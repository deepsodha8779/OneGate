-- Add migration script here


DROP TABLE IF EXISTS  member_details;
CREATE TABLE IF NOT EXISTS member_details
(
    id VARCHAR PRIMARY KEY,
    user_id VARCHAR,
    CONSTRAINT fk_users FOREIGN KEY (user_id) REFERENCES users (id)
);
    




