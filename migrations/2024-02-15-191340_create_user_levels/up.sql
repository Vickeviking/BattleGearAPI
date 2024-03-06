-- User Levels Table
CREATE TABLE User_Levels (
    user_level_id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES Users(user_id),
    level INTEGER,
    experience_points INTEGER DEFAULT 0
);