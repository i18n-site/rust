#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

export RUSTICOS=""
cargo build -Z build-std=panic_abort,std --target wasm32-unknown-unknown --release

TARGET=$(cargo metadata --no-deps --format-version=1 | jq -r .target_directory)
NAME=$(basename $DIR)

rm -rf dist
mkdir -p dist

wasmdir=$TARGET/wasm32-unknown-unknown/release/$NAME
rm -rf $wasmdir
mkdir -p $wasmdir

mv $wasmdir.wasm $wasmdir/_.wasm
wasm-bindgen $wasmdir/_.wasm --out-dir dist --target bundler --weak-refs

if ! [ -x "$(command -v wasm-opt)" ]; then
  cargo install wasm-opt
fi

wasm-opt -Oz -o ./dist/__bg.wasm ./dist/__bg.wasm
bun x cep -c coffee -o dist
