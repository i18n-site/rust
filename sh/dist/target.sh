#!/usr/bin/env bash

set -ex

rustup target list --installed | grep -q $1 && exit 0
rustup target add $1

# if [[ -n $RUST_VER ]]; then
#   rustup toolchain install nightly-$RUST_VER
# else
#   rustup update nightly
# fi
