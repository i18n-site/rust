#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

set -a
. ../../conf/env/smtp.sh
set +a

set -ex

exec cargo test --all-features -- --nocapture
# exec cargo nextest run --all-features --nocapture
