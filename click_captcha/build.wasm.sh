#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

export RUSTFLAGS=""
cargo build --features=wasm -Z build-std=panic_abort,std --target wasm32-unknown-unknown --release
