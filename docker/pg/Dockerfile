FROM postgres:latest

COPY init-schema.sql /docker-entrypoint-initdb.d/
COPY drop-test-schema.sql .
COPY drop-schema.sql .

ENV POSTGRES_USER wiki_user
ENV POSTGRES_PASSWORD test
ENV POSTGRES_DB postgres

EXPOSE 5432
