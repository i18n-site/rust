#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

# source ../i18/env.sh
rm -rf ../../md/.gen
./test.sh
