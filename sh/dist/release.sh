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
  name=$(basename $dir)
  cd $BIN/$name
  txz=$VER.txz
  tar -cJvf $txz .
  b3s $txz
  tar -C . -cf $name.tar $txz $txz.b3s
  rm -rf $txz $txz.b3s
  $DIR/gh.sh $PROJECT/$VER $name.tar
  rm -rf $name.tar
done
