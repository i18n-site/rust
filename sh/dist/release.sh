#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
ROOT=${DIR%/*/*}
set -ex

source project.sh

VER=$($DIR/VER.sh $PROJECT)

BIN=$ROOT/target/bin/$PROJECT

./hsc.sh

cd $BIN

find . -mindepth 1 -maxdepth 1 -type d | while read dir; do
  name=$(basename $dir)
  txz=$VER.txz
  NV=$name.$VER
  rm -rf $NV
  mkdir -p $NV
  tar -C $name -cJvf $NV/$txz .
  hsc $NV/$txz
  tar -C $NV -cf $name.tar .
  $DIR/gh.sh $PROJECT/$VER $name.tar
  rm -rf $NV $name.tar
done
