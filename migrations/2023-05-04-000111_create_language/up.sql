CREATE TABLE language (
    id SERIAL PRIMARY KEY,

    code VARCHAR(10) NOT NULL
);

CREATE INDEX idx_language_code ON language(code);

INSERT INTO language (code)
VALUES ('ua'), ('en');