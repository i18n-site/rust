#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
ROOT=${DIR%/*/*}
set -ex

source project.sh

. $DIR/VER.sh

BIN=$ROOT/target/bin/$PROJECT

./b3s.sh

cd $BIN

find . -mindepth 1 -maxdepth 1 -type d | while read dir; do
  tarname=$(basename tardir).txz
  tar -C $dir -cJvf $tarname .
  b3s $tarname
  $DIR/gh.sh $PROJECT/$VER $tarname*
done
