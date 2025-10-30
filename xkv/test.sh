#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

cleanup() {
  echo "stop container..."
  docker stop xkv_test_db
  echo "container removed"
}

trap cleanup EXIT SIGINT SIGTERM
docker stop xkv_test_db && docker rm xkv_test_db || true

# docker run -d --rm \
#   --name xkv_test_db \
#   -e REDIS_PASSWORD=xxx \
#   -p 6666:6379 \
#   redis

docker run -d --rm \
  --name xkv_test_db \
  -p 6666:6666 apache/kvrocks \
  --bind 0.0.0.0 \
  --resp3-enabled yes --requirepass xxx

set -o allexport
R_PORT=6666
R_NODE="127.0.0.1:$R_PORT"
R_PASSWORD=xxx
R_RESP=3
R_USER=
set +o allexport

cd demo
cargo run
