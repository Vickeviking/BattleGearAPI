-- Friendships Table
CREATE TABLE Friendships (
    friendship_id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES Users(user_id),
    friend_id INTEGER REFERENCES Users(user_id),
    status VARCHAR(50) NOT NULL CHECK (status IN ('pending', 'accepted', 'blocked')),
    friendship_date TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);