#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*/*}
cd $DIR
set -ex

target=$(rustc -vV | grep "host:" | awk '{print $2}')

mkdir -p /so
# cp /usr/lib/libgcc_s.so.* /so
ldd $1 | grep "=> /" | awk '{print $3}' | xargs -I '{}' sh -c 'cp -L "{}" /so/'
