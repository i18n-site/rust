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
  txz=$VER.txz
  NV=$name.$VER
  rm -rf $NV
  mkdir -p $NV
  tar -C $name -cJvf $NV/$txz .
  b3s $NV/$txz
  tar -C $NV -cf $name.tar .
  $DIR/gh.sh $PROJECT/$VER $NV/$name.tar
  rm -rf $NV
done
