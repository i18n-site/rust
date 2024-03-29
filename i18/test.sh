#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

# direnv exec . cargo test --all-features -- --nocapture
direnv exec . cargo run -- -d $(realpath $DIR/../..)/site/i18n
