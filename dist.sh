#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

if [ $# -eq 0 ]; then
  echo miss project
  exit 1
fi

if ! [ -x "$(command -v cargo-v)" ]; then
  cargo install cargo-v
fi

cd $1
bun x mdt .
rm -rf Cargo.lock
ln -s ../Cargo.lock
cargo v patch -y

git describe --tags $(git rev-list --tags --max-count=1) | xargs git tag -d

rm Cargo.lock
git add -u
git commit -m.
git push
cargo publish --registry crates-io || true
