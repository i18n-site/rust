#!/usr/bin/env bash

set -ex

exe=b3s

if ! command -v $exe &>/dev/null; then
  cd /tmp
  down=https://github.com/i18n-site/rust/releases/download
  target=$(rustc -vV | grep "host:" | awk '{print $2}')
  ver=$(curl -fsSL $down/v/$exe)
  file=$target
  tar=$file.tar

  if command -v wget &>/dev/null; then
    get="wget -c -O"
  else
    get="curl -fsSL --retry 9 --retry-all-errors -o"
  fi
  outdir=$down/$exe/$ver
  mkdir -p $outdir
  $get $tar $outdir/$tar
  BIN=/usr/local/bin
  sudo=sudo
  case "$(uname -s)" in
  MINGW*)
    BIN=/usr/bin
    sudo=""
    #BIN=$(dirname $(which bash))
    # chmod +x $file/*
    ;;
  esac
  tar xvf $tar
  $sudo tar xvf $file.txz -C $BIN
  rm -rf $file.*
fi
