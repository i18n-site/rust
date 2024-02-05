#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

. ../dist/project.sh
. ../dist/VER.sh

set -ex
bun i

rm -rf dist
DP=$DIR/dist/$PROJECT
DV=$DP/$VER
mkdir -p $DV
cd $DV
gh release download --clobber $PROJECT/$VER

cd $DIR/dist
mkdir -p v
echo -n $VER >v/$PROJECT

find . -mindepth 1 -maxdepth 1 -type d | while read file; do
  ../rcp.sh $file
done

bun run --bun ../cf.clean.js -- $PROJECT

ap() {
  git init
  cp -f $DIR/conf/git.config .git/config
  git checkout -b $1 || true
  git add . &&
    git commit -m. &&
    git push -f --set-upstream origin $1
}

export GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=accept-new"
cd $DP
ap $PROJECT
cd ../v
ap v
rm -rf $DIR/dist
