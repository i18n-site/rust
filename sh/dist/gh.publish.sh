#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

. project.sh
VER=$(./VER.sh)

gh release edit $PROJECT/$VER --draft=false
