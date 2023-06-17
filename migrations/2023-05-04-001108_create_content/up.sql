CREATE TYPE CONTENT_TYPE AS ENUM ('full', 'diff');

CREATE TABLE version_content (
    id SERIAL PRIMARY KEY,
    
    content BYTEA NOT NULL,
    content_type CONTENT_TYPE NOT NULL,
    content_length INT NOT NULL
);