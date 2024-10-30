#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

MDDIR=~/demo/flashcat/md
# direnv exec . cargo run -- --dir $(realpath $DIR/../..)/md $@
direnv exec . cargo run -- --dir $MDDIR $@
# direnv exec . cargo run -- --dir $(realpath $DIR/../..)/md $@ -n
