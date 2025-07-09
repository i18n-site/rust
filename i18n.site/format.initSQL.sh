#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

if ! command -v pg_format &>/dev/null; then
  brew install pgformatter
fi

pg_format -W8 -f2 -i -s2 src/init.sql
