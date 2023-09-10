CREATE TABLE user_account(
    id SERIAL PRIMARY KEY,
    
    email VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(30) NOT NULL UNIQUE,
    active BOOLEAN DEFAULT false NOT NULL,
    blocked BOOLEAN DEFAULT false NOT NULL,

    role_id INT NOT NULL,
    FOREIGN KEY (role_id) REFERENCES user_role(id),

    updated_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,

    updated_by INT,
    FOREIGN KEY (updated_by) REFERENCES user_account(id)
);

CREATE INDEX idx_user_account_email ON user_account(email);

ALTER SEQUENCE user_account_id_seq RESTART WITH 1000;

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