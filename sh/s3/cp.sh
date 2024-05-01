#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

export GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=accept-new"

. ../dist/project.sh
. ../dist/VER.sh

set -ex
bun i

DIST=$DIR/dist
rm -rf dist
CP=$DIST/cp
mkdir -p $CP
cd dist/cp
PV=$PROJECT/$VER
mkdir -p $PV
cd $PV
gh release download --clobber $PV
cd $CP
mkdir -p v
echo -n $VER >v/$PROJECT
$DIR/rcp.sh .
bun run --bun $DIR/cf.clean.js -- $PROJECT

ap() {
  git init
  cp -f $DIR/conf/git.config .git/config
  git checkout -b $1 || true
  git pull --depth=1 origin $1
}
gpush() {
  git add . &&
    git commit -m. || true
  git push -f --set-upstream origin $1
}

mkdir -p $DIR/dist/git
cd $DIR/dist/git
ap $PROJECT
ls | sort -r -V | tail -n +4 | xargs rm -rf
rm -rf $VER
mv $CP/$PROJECT/$VER .
gpush $PROJECT

mkdir -p $DIST/v
cd $DIST/v
ap v
echo -n $VER >$PROJECT
gpush v

#rm -rf $DIR/dist
