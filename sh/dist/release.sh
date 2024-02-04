#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
ROOT=${DIR%/*/*}
set -ex

source project.sh

. $DIR/VER.sh

BIN=$ROOT/target/bin/$PROJECT

. ./b3s.sh

cd $BIN

find . -mindepth 1 -maxdepth 1 -type d | while read file; do
  tarname=$(basename $file).tar.xz
  tar -cJvf $tarname $file
  # b3s $tarname
  $DIR/gh.sh $PROJECT/$VER $tarname
  # b3sum --raw $tarname >$tarname.b3
  # dist $tarname.b3
done
