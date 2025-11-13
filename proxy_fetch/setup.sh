#!/usr/bin/env bash

set -e
DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -x

cargo build \
  --release \
  --out-dir /opt/rust/bin \
  -Z unstable-options
