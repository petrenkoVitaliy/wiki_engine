CREATE TABLE user_role (
    id SERIAL PRIMARY KEY,
    
    role VARCHAR(255) NOT NULL UNIQUE
);

INSERT INTO user_role (role)
VALUES ('common'), ('moderator'), ('admin');