#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

. ../dist/project.sh
. ../dist/VER.sh

set -ex
bun i

DV=dist/$PROJECT/$VER
mkdir -p $DV
cd $DV
gh release download --clobber $PROJECT/$VER
cd ../..
mkdir -p v
echo $VER >v/$PROJECT

find . -mindepth 1 -maxdepth 1 -type d | while read file; do
  ../rcp.sh $file
done

bun run --bun ../cf.clean.js -- $PROJECT

export GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=accept-new"
git init
cp -f $DIR/conf/git.config .git/config
git add .
git commit -m$VER
git push -f --set-upstream origin main
cd ..
rm -rf dist
