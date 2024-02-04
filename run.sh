#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

name=$1
# name=${1:-rsrv}
# name=$(dasel package.name -f Cargo.toml)
# name=${name//\'/}

exe=./target/debug/$name
rm -rf $exe

cargo build -p $name

./$name/test.sh

# if [ -f "$exe" ]; then
#   GREEN='\033[0;92m'
#   NC='\033[0m'
#   pkill -9 $name || true
#   echo -e "\n${GREEN}❯ $exe$NC\n"
#
# else
#   cd $name
#   exec direnv exec . cargo test -p $name -- --nocapture
# fi
