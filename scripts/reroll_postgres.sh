docker exec wiki_postgres psql postgres wiki_user -f drop-test-schema.sql
env DATABASE_URL="postgresql://wiki_user:test@localhost:5432/postgres?options=-c search_path%3Dmain" diesel migration run