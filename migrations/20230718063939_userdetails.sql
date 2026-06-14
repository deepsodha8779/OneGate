CREATE TYPE contact_types AS ENUM ('mobile', 'home', 'work');
CREATE TABLE IF NOT EXISTS user_details (
    id VARCHAR PRIMARY KEY,
    first_name VARCHAR(50),
    last_name VARCHAR(50),
    aadhar_number VARCHAR(20) UNIQUE,
    photo_url VARCHAR,
    contact_number VARCHAR(20),
    contact_type contact_types, 
    email VARCHAR(100) UNIQUE
);

