-- This file should undo anything in `up.sql`
ALTER TABLE Users
DROP CONSTRAINT username_unique;

ALTER TABLE Users
ALTER COLUMN password_hash TYPE VARCHAR(60);