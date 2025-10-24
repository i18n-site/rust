#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

./minify.py
b2 file upload --content-type text/js i-i18n minify.sh i
