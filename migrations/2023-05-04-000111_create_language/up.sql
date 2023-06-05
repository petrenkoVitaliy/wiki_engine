CREATE TABLE language (
    id SERIAL PRIMARY KEY,

    code VARCHAR(10) NOT NULL
);

INSERT INTO language (code)
VALUES ('ua'), ('en');