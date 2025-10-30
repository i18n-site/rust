#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

./wasm.sh
bun x cep -c test.coffee && node --experimental-wasm-modules test.js
