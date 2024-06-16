#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

if [ -z "$1" ]; then
  PROJECT=$(realpath $DIR)/tests/yml
else
  PROJECT=$1
fi

# direnv exec . cargo test --all-features -- --nocapture
# direnv exec . cargo run -- -d $PROJECT
#direnv exec . cargo run -- -d $PROJECT
direnv exec . cargo run -- -d ../../md --purge
# direnv exec . cargo run -- -d ~/i18n/plugin
# direnv exec . cargo run -- -d ~/i18n/site
# direnv exec . cargo run -- -d ~/i18n/srv/mod
