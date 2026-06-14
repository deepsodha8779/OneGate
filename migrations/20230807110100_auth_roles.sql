-- Add migration script here
-- Add migration script here
DROP TABLE IF EXISTS  auth_roles;
CREATE TABLE IF NOT EXISTS auth_roles
(
    id VARCHAR PRIMARY KEY,
    user_auth_id VARCHAR,
    role  role,
    CONSTRAINT fk_user_auth FOREIGN KEY (user_auth_id) REFERENCES userauth (id)
);

