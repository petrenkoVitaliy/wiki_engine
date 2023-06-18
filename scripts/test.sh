while getopts ":f" option; do
  case $option in
    f)
        echo "[SCRIPT] truncating test schema..."
        docker exec wiki_postgres psql postgres wiki_user -f drop-test-schema.sql

        echo "[SCRIPT] migrating test schema..."
        env DATABASE_URL="postgresql://wiki_user:test@localhost:5432/postgres?options=-c search_path%3Dtest" diesel migration run
    ;;
  esac
done

env ROCKET_PROFILE=test cargo test -- --nocapture