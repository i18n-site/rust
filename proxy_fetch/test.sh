#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -a
. ~/.config/ss_subscription
set +a
set -ex

cargo test --all-features -- --nocapture
