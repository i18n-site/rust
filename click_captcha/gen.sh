#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR
set -ex

cd gen
bun i
./svg-compress.sh
./gen.coffee
