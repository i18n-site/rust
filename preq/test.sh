#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -e
set -a
. ~/.config/ipv6_proxy.env
set +a
set -x
cargo test -- --nocapture
