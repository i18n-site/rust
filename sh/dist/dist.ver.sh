#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

. project.sh
. VER.sh

mkdir -p v
cd v
echo $VER >$PROJECT
$DIR/gh.publish.sh $PROJECT
gh release delete-asset v $PROJECT || true
$DIR/gh.sh v $PROJECT || (gh release create v -n v || true) && $DIR/gh.sh v $PROJECT
cd ..
rm -rf v
