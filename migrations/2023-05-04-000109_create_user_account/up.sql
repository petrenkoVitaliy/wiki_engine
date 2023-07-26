CREATE TABLE user_account(
    id SERIAL PRIMARY KEY,
    
    email VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL UNIQUE,
    active BOOLEAN DEFAULT true NOT NULL,

    role_id INT NOT NULL,
    FOREIGN KEY (role_id) REFERENCES user_role(id),

    updated_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL
);

CREATE  FUNCTION update_user_account_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER user_account_on_update
    BEFORE UPDATE
    ON user_account
    FOR EACH ROW
EXECUTE PROCEDURE update_user_account_updated_at();