#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

bash <(curl -sSL https://registry.npmmirror.com/@i18n.site/i/latest/files/i) i18
