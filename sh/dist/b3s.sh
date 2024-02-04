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
  BIN=/usr/local/bin
  case "$(uname -s)" in
  MINGW*)
    BIN=$(dirname $(which bash))
    chmod +x $file/*
    ;;
  Darwin)
    sudo=sudo
    ;;
  esac
  $sudo mv $file/* $BIN
  rm -rf $file $txz
fi
