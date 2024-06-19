#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

cat pkg/package.json | jq -r '.version'

curl https://registry.npmmirror.com/@i18n.site/i/latest/files/i
