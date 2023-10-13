#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR
set -ex

./svg-compress.sh
./svg-D.gen.coffee
