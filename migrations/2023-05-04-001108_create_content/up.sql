CREATE TYPE CONTENT_TYPE AS ENUM ('Full', 'Diff');

CREATE TABLE version_content (
    id SERIAL PRIMARY KEY,
    
    content BYTEA NOT NULL,
    content_type CONTENT_TYPE NOT NULL,
    
    updated_at TIMESTAMP DEFAULT NOW(),
    created_at TIMESTAMP DEFAULT NOW()
);