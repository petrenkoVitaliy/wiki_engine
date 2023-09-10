CREATE TABLE user_otp (
    id SERIAL PRIMARY KEY,
    
    user_id INT NOT NULL UNIQUE,
    FOREIGN KEY (user_id) REFERENCES user_account(id) ON DELETE CASCADE,

    otp VARCHAR(255) NOT NULL,

    created_at TIMESTAMP DEFAULT NOW() NOT NULL
);