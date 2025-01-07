#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

if [ ${#1} -eq 0 ]; then
  if [ -f ".dev" ]; then
    arg=$(cat .dev)
  else
    echo "❯ $0 项目名"
    exit 1
  fi
else
  echo $@ >.dev
  arg=$@
fi

source ./sh/pid.sh

set -ex

# if ! [ -x "$(command -v dasel)" ]; then
#   go install github.com/tomwright/dasel/v2/cmd/dasel@master
# fi
# [[ -d target ]] && cargo sweep --time 30 && cargo sweep --installed

exec watchexec \
  --shell=none \
  --project-origin . -w . \
  --exts rs,toml \
  -r \
  -- ./run.sh $arg
