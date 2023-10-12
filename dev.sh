#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

source ./sh/pid.sh

export RUST_BACKTRACE=short
export RUSTFLAGS='--cfg reqwest_unstable'
export RUST_LOG=$RUST_LOG,watchexec=off,watchexec_cli=off,globset=warn

if ! [ -x "$(command -v dasel)" ]; then
  go install github.com/tomwright/dasel/v2/cmd/dasel@master
fi

exec watchexec \
  --shell=none \
  --project-origin . -w . \
  --exts rs,toml \
  -r \
  -- ./run.sh $@
