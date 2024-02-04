#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

. project.sh
. VER.sh

echo $VER
echo $VER >v
$DIR/gh.publish.sh $PROJECT
gh release delete-asset $PROJECT v || true
$DIR/gh.sh $PROJECT v
rm v
