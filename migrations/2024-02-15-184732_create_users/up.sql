-- Your SQL goes here

/* Create the users table */
CREATE TABLE Users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(254) UNIQUE NOT NULL,
    password_hash VARCHAR(60) NOT NULL,
    auth_token VARCHAR(500),
    full_name VARCHAR(255) NOT NULL,
    avatar_id INTEGER REFERENCES Images(image_id),
    registration_date TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE,
    is_admin BOOLEAN DEFAULT FALSE,
    timezone VARCHAR(50) DEFAULT 'UTC',
    language VARCHAR(50) DEFAULT 'English',
    country VARCHAR(50) DEFAULT 'USA',
    date_of_birth DATE NOT NULL,
    two_factor_auth_enabled BOOLEAN DEFAULT FALSE,
    last_password_change TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

