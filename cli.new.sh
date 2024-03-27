#!/usr/bin/env bash

DIR=$(realpath ${0%/*})
cd $DIR
set -ex

cargo new --bin $1 # this will add lib to workspace
rm -rf $1

cp -R cli_tmpl $1

cd $1

rpl cli_tmpl $1

cd $DIR/.github/workflows
cp i18.yml $1.yml

sd "i18" "$1" $1.yml

cd $DIR
git add .
