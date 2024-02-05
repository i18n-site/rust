#!/usr/bin/env bash

ROOT=$(realpath $0) && ROOT=${ROOT%/*}
DIST=$ROOT/sh/dist
source $DIST/project.sh
cd $ROOT

set -ex

git add . && git commit -m. || true

branch=$(git symbolic-ref --short -q HEAD || echo main)
if [ "$branch" != "main" ]; then
  git fetch origin $branch
  git merge origin/$branch -m merge
fi

$ROOT/cargo.dist.sh $PROJECT
beginhash=$(git log --format=%H -1 main)

source $DIST/VER.sh
PV=$PROJECT/$VER

if [ "$branch" != "main" ]; then
  git fetch origin main
  git merge origin/main -m merge
  git merge main -m merge
  git checkout main
  git merge $branch -m merge
fi

cd $ROOT
git reset --soft $beginhash || true
git add .
git commit -m $PV || true

git tag $PV
git push origin main
git push origin $PV

if [ "$branch" != "main" ]; then
  git checkout $branch
  git reset --hard $(git log --format=%H -1 main)
  git add .
  git commit -m $PV || true
  git push -f
fi
