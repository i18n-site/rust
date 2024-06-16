#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

source ./sh/cflag.sh

cargo build $RUST_FEATURES --release --target $RUST_TARGET
