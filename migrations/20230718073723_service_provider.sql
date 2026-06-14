CREATE TYPE serviceprovidertypes AS ENUM ('other', 'vendor','milkman', 'maid','laundry','driver','cook','carcleaner');
CREATE TABLE IF NOT EXISTS service_provider
(
    id VARCHAR PRIMARY KEY,
    user_id VARCHAR,
    service_provider_types serviceprovidertypes,
    CONSTRAINT fk_users FOREIGN KEY (user_id) REFERENCES users (id)
);