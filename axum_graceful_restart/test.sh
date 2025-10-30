#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

cd ./tests

TARGET_DIR=$(cargo metadata --no-deps --format-version=1 | jq -r '.target_directory')

cargo build --release

cd ..

exec $TARGET_DIR/release/axum_graceful_restart_tests
