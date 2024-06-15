#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

cd ./test_cget
cargo run -p test_cget -- -c -k xx.sk test.xz
