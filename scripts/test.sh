source "$(dirname "$0")/.utils.sh"

cov=false;

while getopts ":f :c" option; do
  case $option in
    f)
      print_log "truncating test schema:"
      docker exec wiki_postgres psql postgres wiki_user -f drop-test-schema.sql

      print_log "migrating test schema:"
      env DATABASE_URL="postgresql://wiki_user:test@localhost:5432/postgres?options=-c search_path%3Dtest" diesel migration run
      ;;
    c)
      cov=true;
      ;;
  esac
done

if $cov; then
    print_log "running tests (cov):"
    
    CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='coverage/cargo-test-%p-%m.profraw' ROCKET_PROFILE=test cargo test
else
  print_log "running tests:"

  ROCKET_PROFILE=test cargo test
fi

