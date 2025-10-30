#!/usr/bin/env bash

DIR=$(realpath ${0%/*})
cd $DIR

if [ ! -n "$1" ]; then
  echo "USAGE : $0 project_name"
  exit 1
fi

set -ex

cargo new --lib $1 # this will add lib to workspace

rm -rf $1

cp -R _tmpl $1

cd $1

rpl _tmpl $1

git add .
