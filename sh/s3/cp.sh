#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

. ../dist/project.sh
. ../dist/VER.sh

set -ex

DV=dist/$PROJECT/$VER
mkdir -p $DV
cd $DV
gh release download --clobber $PROJECT/$VER
cd ../..
mkdir -p v
echo $VER >v/$PROJECT

find . -mindepth 1 -maxdepth 1 -exec ../rcp.sh {} \;

cd ..

rm -rf dist
