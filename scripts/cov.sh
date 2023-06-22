source "$(dirname "$0")/.utils.sh"

print_log "removing coverage folder:"
rm -rf ./coverage
mkdir ./coverage

print_log "running tests:"
while getopts ":f" option; do
  case $option in
    f)
        print_log "truncating test schema:"
        docker exec wiki_postgres psql postgres wiki_user -f drop-test-schema.sql

        print_log "migrating test schema:"
        env DATABASE_URL="postgresql://wiki_user:test@localhost:5432/postgres?options=-c search_path%3Dtest" diesel migration run
    ;;
  esac
done
CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='coverage/cargo-test-%p-%m.profraw' ROCKET_PROFILE=test cargo test

print_log "generating coverage:"
grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o coverage/html        

print_log "opening:"
open ./coverage/html/index.html

# lcov
# grcov . --binary-path ./target/debug/deps/ -s . -t lcov --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/tests.lcov