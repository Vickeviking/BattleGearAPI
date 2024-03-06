-- Trophies Table
DROP TABLE IF EXISTS Trophies;
CREATE TABLE Trophies (
    trophy_id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES Users(user_id),
    points INTEGER,
    game_timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);