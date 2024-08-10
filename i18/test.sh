#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

if [ -z "$1" ]; then
  PROJECT=$(realpath $DIR)/tests
else
  PROJECT=$1
fi

# direnv exec . cargo test --all-features -- --nocapture

# direnv exec . cargo run -- -d ../../md --purge
direnv exec . cargo run -- -d ../../md
# direnv exec . cargo run -- -d $PROJECT
# direnv exec . cargo run -- -d $PROJECT
