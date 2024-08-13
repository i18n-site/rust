#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

find . -type f -name "*.rs" | grep -v /tests/ | grep -iv todo | xargs -I {} sed -i '/dbg\!/d' {}
