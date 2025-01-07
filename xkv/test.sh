#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

cargo build

RUST_BACKTRACE=short \
  exec cargo test -- --nocapture
