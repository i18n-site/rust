#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

if ! command -v cargo-binstall &>/dev/null; then
  curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
fi

ensure() {
  for i in "$@"; do
    if ! command -v cargo-$i &>/dev/null; then
      cargo binstall cargo-$1 -y --force
    fi
  done
}

ensure sweep nextest
