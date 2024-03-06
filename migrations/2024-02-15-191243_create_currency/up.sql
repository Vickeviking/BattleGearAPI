-- Currency Table
CREATE TABLE Currency (
    currency_id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES Users(user_id),
    currency_type VARCHAR(50),
    amount INTEGER DEFAULT 0,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);