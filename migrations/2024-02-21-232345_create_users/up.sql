-- Your SQL goes here
ALTER TABLE Users
ADD CONSTRAINT username_unique UNIQUE (username);

ALTER TABLE Users
ALTER COLUMN password_hash TYPE VARCHAR(128);