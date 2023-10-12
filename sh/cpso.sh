#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*/*}
cd $DIR
set -ex

if ! [ -x "$(command -v cargo)" ]; then
  cargo_env="$HOME/.cargo/env"
  if [ -f "$cargo_env" ]; then
    source $cargo_env
  fi
fi

cpso() {
  mkdir -p /so
  ldd $1 | grep "=> /" | awk '{print $3}' | xargs -I '{}' sh -c 'cp -L "{}" /so/'
  # cp ${1%.so}* /so${sodir}
}

target=$(rustc -vV | grep "host:" | awk '{print $2}')

name=$(grep "^name" Cargo.toml | sed 's/name = //g' | awk -F\" '{print $2}')

mv target/$target/release/$name target/app
cpso target/app
