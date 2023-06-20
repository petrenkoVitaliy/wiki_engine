CREATE TABLE article (
    id SERIAL PRIMARY KEY,

    enabled BOOLEAN DEFAULT true,
    archived BOOLEAN DEFAULT false,
    
    updated_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW()
);