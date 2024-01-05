#!/usr/bin/env bash

DIR=$(realpath ${0%/*})
cd $DIR
set -ex

npm patch -y
npm publish --access=public
