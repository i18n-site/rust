#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

if [ $# -eq 0 ]; then
  echo "usage: $0 <project>"
  exit 1
fi

set -ex
rm -rf Cargo.lock

if ! [ -x "$(command -v cargo-v)" ]; then
  cargo install cargo-v
fi

cargo build -p $(basename $1)

./clippy.sh

cd $1

bun x mdt .
ln -s ../Cargo.lock
cargo v patch -y

git describe --tags $(git rev-list --tags --max-count=1) | xargs git tag -d

rm Cargo.lock
git add -u
git commit -m. || true
git push
cargo publish --registry crates-io || true
cd $DIR
direnv exec . ./sh/upgrade.coffee
rm Cargo.lock
git add -u
git commit -m. || true
