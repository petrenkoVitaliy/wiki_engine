CREATE TABLE article_language (
    id SERIAL PRIMARY KEY,
    
    name VARCHAR(255) NOT NULL,

    enabled BOOLEAN DEFAULT true NOT NULL,
    archived BOOLEAN DEFAULT false NOT NULL,
    
    article_id INTEGER NOT NULL,
    FOREIGN KEY (article_id) REFERENCES article(id) ON DELETE CASCADE,

    language_id INTEGER NOT NULL,
    FOREIGN KEY (language_id) REFERENCES language(id) ON DELETE CASCADE,

    updated_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,

    updated_by INT,
    FOREIGN KEY (updated_by) REFERENCES user_account(id),
    created_by INT NOT NULL,
    FOREIGN KEY (created_by) REFERENCES user_account(id),

    CONSTRAINT language_per_article UNIQUE (article_id, language_id)
);

CREATE  FUNCTION update_article_language_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER article_language_on_update
    BEFORE UPDATE
    ON article_language
    FOR EACH ROW
EXECUTE PROCEDURE update_article_language_updated_at();