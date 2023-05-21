-- Your SQL goes here
CREATE TABLE article_language (
    id SERIAL PRIMARY KEY,
    
    name VARCHAR(255) NOT NULL,

    enabled BOOLEAN DEFAULT true,
    archived BOOLEAN DEFAULT false,
    
    article_id INTEGER NOT NULL,
    FOREIGN KEY (article_id) REFERENCES article(id) ON DELETE CASCADE,

    language_id INTEGER NOT NULL,
    FOREIGN KEY (language_id) REFERENCES language(id) ON DELETE CASCADE,

    updated_at TIMESTAMP DEFAULT NOW(),
    created_at TIMESTAMP DEFAULT NOW()
);