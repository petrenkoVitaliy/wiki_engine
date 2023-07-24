CREATE TYPE ARTICLE_TYPE AS ENUM ('private', 'public', 'protected', 'restricted');

CREATE TABLE article (
    id SERIAL PRIMARY KEY,

    enabled BOOLEAN DEFAULT true NOT NULL,
    archived BOOLEAN DEFAULT false NOT NULL,
    article_type ARTICLE_TYPE NOT NULL,
    
    updated_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL
);

CREATE  FUNCTION update_article_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER article_on_update
    BEFORE UPDATE
    ON article
    FOR EACH ROW
EXECUTE PROCEDURE update_article_updated_at();