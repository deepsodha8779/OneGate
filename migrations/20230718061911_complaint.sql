-- Add migration script here

DROP TABLE IF EXISTS  complaint;
CREATE TABLE IF NOT EXISTS complaint
(
    id VARCHAR PRIMARY KEY,
    complaint_by VARCHAR,
    society_id VARCHAR,
    complaint_title VARCHAR,
    complaint_description  VARCHAR,
    status VARCHAR,
    CONSTRAINT fk_society FOREIGN KEY (society_id) REFERENCES society (id)
);




