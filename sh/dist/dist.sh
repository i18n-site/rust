#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

set -ex

source project.sh
./cross.sh $PROJECT
./release.sh $PROJECT
