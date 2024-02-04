#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

source ./docker.build.sh

docker stop $name >/dev/null 2>&1 || true
docker run \
  -v ./.env:/.env \
  -p $PORT:$PORT --name $name --rm $name
