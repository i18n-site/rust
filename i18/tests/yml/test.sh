#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

# rm -rf .gen
echo $API
../../test.sh $DIR
tail -20 en/*
