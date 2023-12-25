#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex
cd init
cargo run -p init
