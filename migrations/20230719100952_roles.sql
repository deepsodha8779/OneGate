-- Add migration script here
DROP TABLE IF EXISTS  roles;
CREATE TYPE role AS ENUM ('admin', 'user','guest', 'maid','serviceprovider','security','member','superadmin');
CREATE TABLE IF NOT EXISTS roles
(
    id VARCHAR PRIMARY KEY,
    user_id VARCHAR,
    role  role,
    CONSTRAINT fk_users FOREIGN KEY (user_id) REFERENCES users (id)
);

