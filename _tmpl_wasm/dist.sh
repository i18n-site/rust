#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

./wasm.sh
git add -u && git commit -m. || true
cd dist
npm version patch
npm publish --access=public
cp package.json ../coffee/
git add -u && git commit -m. || true
