#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

if ! [ -x "$(command -v cargo-v)" ]; then
  cargo install cargo-v
fi

cd $1
rm Cargo.lock
ln -s ../Cargo.lock
cargo v patch -y
rm Cargo.lock
git add -u
git commit -m.
git push
cargo publish --registry crates-io || true
