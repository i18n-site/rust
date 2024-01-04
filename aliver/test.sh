#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

exec curl -X POST -H "Content-Type: application/json" -d @test.json http://127.0.0.1:$PORT
