docker build --rm -t wiki_postgres ./docker/pg
docker run --name wiki_postgres -d -p 5432:5432 wiki_postgres 