CREATE TABLE article_version (
    id SERIAL PRIMARY KEY,
    
    version INT NOT NULL,
    enabled BOOLEAN DEFAULT true NOT NULL,
    name VARCHAR(30) NOT NULL,

    content_id INT NOT NULL,
    FOREIGN KEY (content_id) REFERENCES version_content(id) ON DELETE CASCADE,

    article_language_id INT NOT NULL,
    FOREIGN KEY (article_language_id) REFERENCES article_language(id) ON DELETE CASCADE,

    updated_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,

    updated_by INT,
    FOREIGN KEY (updated_by) REFERENCES user_account(id),
    created_by INT NOT NULL,
    FOREIGN KEY (created_by) REFERENCES user_account(id),

    CONSTRAINT version_per_article_language UNIQUE (version, article_language_id)
);

ALTER SEQUENCE article_version_id_seq RESTART WITH 1000;

CREATE FUNCTION update_article_version_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER article_version_on_update
    BEFORE UPDATE
    ON article_version
    FOR EACH ROW
EXECUTE PROCEDURE update_article_version_updated_at();