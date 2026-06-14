
CREATE TABLE IF NOT EXISTS users (
    id VARCHAR PRIMARY KEY,
    user_detail_id VARCHAR,
    society_id VARCHAR,
    is_block BOOLEAN,
    is_deleted BOOLEAN,
    CONSTRAINT fk_user_details FOREIGN KEY (user_detail_id) REFERENCES user_details (id),
    CONSTRAINT fk_society FOREIGN KEY (society_id) REFERENCES society (id)
);


    