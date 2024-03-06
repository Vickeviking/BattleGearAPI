-- Your SQL goes here
CREATE TABLE users_roles (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES Users(user_id),
    role_id INTEGER NOT NULL REFERENCES Roles(id)
);