-- Add migration script here

DROP TABLE IF EXISTS  notice;
CREATE TABLE IF NOT EXISTS notice
(
    id VARCHAR PRIMARY KEY,
    society_id VARCHAR,
    title VARCHAR,
    notice_description  VARCHAR,
    CONSTRAINT fk_society FOREIGN KEY (society_id) REFERENCES society (id)
);





