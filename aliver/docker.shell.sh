#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex
source ./docker.build.sh
docker stop $name >/dev/null 2>&1 || true
docker run -it --entrypoint /bin/sh \
  -v ./.env:/.env \
  --name $name --rm $name
