#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

cargo test --all-features -- --nocapture 2>&1 | tee out.txt
