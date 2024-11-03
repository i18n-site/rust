#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

if [ -f "env.sh" ]; then
  . env.sh
fi

export MREQ_PROTOCOL=http
export API=$API_HOST:8850
export MDDIR=$HOME/demo/flashcat/md
# cargo run -- --dir $(realpath $DIR/../..)/md $@
cargo run -- --dir $MDDIR -n
# cargo run -- --dir $(realpath $DIR/../..)/md $@ -n
