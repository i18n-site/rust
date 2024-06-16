#!/usr/bin/env bash

ROOT=$(realpath $0) && ROOT=${ROOT%/*}
DIST=$ROOT/sh/dist
cd $ROOT

set -ex

git add . && git commit -m. || true

branch=$(git symbolic-ref --short -q HEAD || echo main)
if [ "$branch" != "main" ]; then
  git fetch origin $branch
  git merge origin/$branch -m merge
fi

$ROOT/cargo.dist.sh $@

dist() {
  VER=$($DIST/VER.sh $1)
  PV=$1/$VER
  beginhash=$(git log --format=%H -1 main)

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

  push() {
    git push $1 main
    git push $1 $PV
  }

  push origin
  push github

  if [ "$branch" != "main" ]; then
    git checkout $branch
    git reset --hard $(git log --format=%H -1 main)
    git add .
    git commit -m $PV || true
    git push -f
  fi
}

for arg in "$@"; do
  dist $arg
done
