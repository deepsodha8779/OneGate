--- Add migration script here

-- DROP TABLE IF EXISTS  security_details;
CREATE TABLE IF NOT EXISTS security_details
(
    id VARCHAR PRIMARY KEY,
    user_id VARCHAR,
    start_time VARCHAR,
    end_time  VARCHAR,
    CONSTRAINT fk_users FOREIGN KEY (user_id) REFERENCES users (id)
);


