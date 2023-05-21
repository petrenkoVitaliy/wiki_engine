-- Your SQL goes here
CREATE TABLE article_version (
    id SERIAL PRIMARY KEY,
    
    version INT NOT NULL,
    content VARCHAR NOT NULL,

    enabled BOOLEAN DEFAULT true,
    
    article_language_id INT NOT NULL,
    FOREIGN KEY (article_language_id) REFERENCES article_language(id) ON DELETE CASCADE,

    updated_at TIMESTAMP DEFAULT NOW(),
    created_at TIMESTAMP DEFAULT NOW()
);