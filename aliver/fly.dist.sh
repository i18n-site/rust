#!/usr/bin/env bash

DIR=$(realpath ${0%/*})
cd $DIR
set -ex

cd ~/.fly
cp -f iuser.config.yml config.yml
if ! command -v fly &>/dev/null; then
  curl -L https://fly.io/install.sh | sh
fi
cd $DIR
fly deploy
