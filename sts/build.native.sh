#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

export NATIVE=1

source ./sh/cflag.sh

cargo build $RUST_FEATURES --release --target $RUST_TARGET
