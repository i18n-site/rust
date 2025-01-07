#!/usr/bin/env bash

# 检测未使用的依赖

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

set -ex

if ! hash cargo-machete 2>/dev/null; then
  cargo install cargo-machete
fi

cd ..
cargo machete --fix

# if ! hash cargo-udeps 2>/dev/null; then
#   cargo install cargo-udeps --locked
# fi
#
# cargo +nightly udeps --workspace --all-features --output json | direnv exec . ./udeps.coffee
