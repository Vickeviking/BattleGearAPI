-- Your SQL goes here
CREATE TABLE Roles (
    id SERIAL PRIMARY KEY,
    code varchar(64) NOT NULL UNIQUE,
    name varchar(128) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);