-- Your SQL goes here

CREATE TABLE Total_Throphies (
    total_throphies_id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES Users(user_id),
    total INT DEFAULT 0
);