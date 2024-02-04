#!/usr/bin/env bash

set -ex

exe=b3s

if ! command -v $exe &>/dev/null; then
  cd /tmp
  down=https://github.com/i18n-site/rust/releases/download
  target=$(rustc -vV | grep "host:" | awk '{print $2}')
  ver=$(curl -fsSL $down/v/$exe)
  file=$target
  txz=$file.tar.xz
  curl -o $txz -fsSL $down/$exe/$ver/$txz
  tar xvf $txz
  BIN=$HOME/.bin
  mkdir -p $BIN
  export PATH=$BIN:$PATH
  mv $file/$exe $BIN
  rm -rf $file $txz
fi
