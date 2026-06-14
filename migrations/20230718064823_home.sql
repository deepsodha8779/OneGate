-- Add migration script here


DROP TABLE IF EXISTS  home;
CREATE TABLE IF NOT EXISTS home
(
    id VARCHAR PRIMARY KEY,
    society_id VARCHAR,
    wing VARCHAR,
    no  INT,  
    CONSTRAINT fk_society FOREIGN KEY (society_id) REFERENCES society (id)
);



