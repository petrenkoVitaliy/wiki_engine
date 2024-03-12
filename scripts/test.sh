#!/bin/bash
. ./scripts/.utils.sh


# USAGE:
# sh scripts/test.sh
# Options:
# -f   format DB 
# -c   with coverage

coverage=false;

while getopts ":f :c" option; do
  case $option in
    f)
      print_log "truncating test schema:"
      docker exec wiki_postgres psql postgres wiki_user -f drop-test-schema.sql

      print_log "migrating test schema:"
      env DATABASE_URL="postgresql://wiki_user:test@localhost:5432/postgres?options=-c search_path%3Dtest" diesel migration run
      ;;
    c)
      coverage=true;
      ;;
  esac
done

if $coverage; then
    print_log "running tests (coverage):"

    cargo llvm-cov --open

    # ALTERNATIVELY:
    
    # CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='coverage/cargo-test-%p-%m.profraw' ROCKET_PROFILE=test cargo test
    
    # if not exist => cargo install grcov
    # grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o coverage/html        

    # print_log "opening:"
    # open ./coverage/html/index.html

else
  print_log "running tests:"

  ROCKET_PROFILE=test cargo test
fi

