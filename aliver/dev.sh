#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

source ./sh/pid.sh

set -ex

exec watchexec \
  --shell=none \
  --project-origin . \
  -w src/ \
  --exts rs,toml,proto \
  -r \
  -- ./run.sh
