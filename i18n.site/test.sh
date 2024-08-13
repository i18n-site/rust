#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

# direnv exec . cargo run -- --s3 --dir $(realpath $DIR/../..)/md

# direnv exec . cargo run -- --dir $(realpath $DIR/../..)/md $@
# direnv exec . cargo run -- --dir $(realpath $DIR/../..)/md $@

# direnv exec . cargo run -- --dir $(realpath $DIR/../..)/md $@
direnv exec . cargo run -- --dir $(realpath $DIR/../..)/md $@ #-n
