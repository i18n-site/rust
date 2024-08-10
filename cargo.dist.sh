#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

if [ $# -eq 0 ]; then
  echo "usage: $0 <project>"
  exit 1
fi

git pull

dist() {
  name=$(basename $1 | sed 's/\./-/g')

  cargo build -p $name

  cd $1

  bun x mdt .
  rm -rf Cargo.lock
  ln -s ../Cargo.lock
  cargo v patch -y

  git describe --tags $(git rev-list --tags --max-count=1) | xargs git tag -d

  rm Cargo.lock
  git add -u
  git commit -m. || true
  git push
  cargo publish --registry crates-io || true
  cd $DIR
  mise exec -- ./sh/upgrade.coffee
  rm Cargo.lock
  git add -u
  gme $(cargo metadata --format-version=1 --no-deps | jq '.packages[] | .name + ":" + .version' -r | grep "$name:") || true

}

set -ex

rm -rf Cargo.lock
./clippy.sh

if ! [ -x "$(command -v cargo-v)" ]; then
  cargo install cargo-v
fi

for arg in "$@"; do
  dist $arg
done
