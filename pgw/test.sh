#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

set -a
. ../../conf/env/state.sh
set +a

set -ex

cargo test -- --nocapture
