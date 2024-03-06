-- Chats Table
CREATE TABLE Chats (
    chat_id SERIAL PRIMARY KEY,
    sender_id INTEGER REFERENCES Users(user_id),
    receiver_id INTEGER REFERENCES Users(user_id),
    message VARCHAR(1000),
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    is_read BOOLEAN DEFAULT FALSE
);