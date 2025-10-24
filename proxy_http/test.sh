#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -a
. ~/.config/proxy_http
set +a
set -ex

cargo test --all-features -- --nocapture
