docker build --rm -t wiki_postgres ./docker
docker run -d -p 5432:5432 wiki_postgres
