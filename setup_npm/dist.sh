#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

git pull
cd pkg
npm version patch
ver=$(cat package.json | jq -r '.version')
cd ..
git add -u
git commit -m"$(basename $(pwd)) v$ver" || true
git push
cargo run
