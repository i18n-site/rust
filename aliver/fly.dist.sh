#!/usr/bin/env bash

DIR=$(realpath ${0%/*})
cd $DIR
set -ex

./fly.config.sh
if ! command -v fly &>/dev/null; then
  curl -L https://fly.io/install.sh | sh
fi
cd $DIR
fly deploy
