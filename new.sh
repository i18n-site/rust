#!/usr/bin/env bash

DIR=$(realpath ${0%/*})
cd $DIR
set -ex

cargo new --lib $1 # this will add lib to workspace
rm -rf $1

cp -R tmpl $1

cd $1

rpl tmpl $1

git add .
