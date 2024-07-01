#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

# curl -sSL https://unpkg.com/@i18n.site/i/i
curl -sSL https://registry.npmmirror.com/@i18n.site/i/latest/files/i
