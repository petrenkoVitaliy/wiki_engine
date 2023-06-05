CREATE TABLE article_version (
    id SERIAL PRIMARY KEY,
    
    version INT NOT NULL,
    enabled BOOLEAN DEFAULT true,

    content_id INT NOT NULL,
    FOREIGN KEY (content_id) REFERENCES version_content(id) ON DELETE CASCADE,

    article_language_id INT NOT NULL,
    FOREIGN KEY (article_language_id) REFERENCES article_language(id) ON DELETE CASCADE,

    updated_at TIMESTAMP DEFAULT NOW(),
    created_at TIMESTAMP DEFAULT NOW(),

    CONSTRAINT version_per_article_language UNIQUE (version, article_language_id)
);