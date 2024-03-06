CREATE TABLE Images (
    image_id SERIAL PRIMARY KEY,
    image_url VARCHAR(255) NOT NULL,
    description TEXT,
    upload_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);