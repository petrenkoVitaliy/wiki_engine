CREATE TABLE user_role (
    id SERIAL PRIMARY KEY,
    
    role VARCHAR(30) NOT NULL UNIQUE
);

INSERT INTO user_role (id, role)
VALUES (1, 'common'), (2, 'moderator'), (3, 'admin');