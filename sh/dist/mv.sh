#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*/*/*}
cd $DIR
set -ex

ver=$1
arch=$2

TARGET=$DIR/target

BIN=$TARGET/bin

OUT=$BIN/$PROJECT/$arch

rm -rf $OUT

mkdir -p $OUT

find $TARGET/$arch/release -maxdepth 1 -type f -perm 755 | while read file; do
  # if [ "$(uname -s)" == "Darwin" ]; then
  #   # 很奇怪, 不这么mac运行会报错
  #   strip $file
  # fi
  mv "$file" $OUT/
done
