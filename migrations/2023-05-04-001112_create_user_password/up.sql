CREATE TABLE user_password (
    id SERIAL PRIMARY KEY,
    
    user_id INT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user_account(id) ON DELETE CASCADE,

    password VARCHAR(255) NOT NULL,

    updated_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL
);

CREATE  FUNCTION update_user_password_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER user_password_on_update
    BEFORE UPDATE
    ON user_password
    FOR EACH ROW
EXECUTE PROCEDURE update_user_password_updated_at();