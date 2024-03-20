#!/usr/bin/env bash

set -ex

name=i18

DEFAULT_BIN=/usr/local/bin

case "$(uname -s)" in
"Darwin")
  OS="apple-darwin"
  ;;
"Linux")
  (ldd --version 2>&1 | grep -q musl) && clib=musl || clib=gun
  OS="unknown-linux-$clib"
  ;;
"MINGW*" | "CYGWIN*")
  OS="pc-windows-msvc"
  DEFAULT_BIN=$(dirname $(which bash))
  ;;
*)
  echo "Unsupported System"
  exit 1
  ;;
esac

if [ -z "$BIN" ]; then
  BIN=$DEFAULT_BIN
fi

ARCH=$(uname -m)

if [[ "$ARCH" == "arm64" || "$ARCH" == "arm" ]]; then
  ARCH="aarch64"
fi

MIRROR="atomgit.com/i18n-site/dist/raw github.com/i18n-site/rust/releases/download codeberg.org/i18n-site/dist/raw/branch raw.githubusercontent.com/i18n-site/dist bitbucket.org/i18nsite/dist/raw xxai.eu.org"

CURL="curl -L --connect-timeout 9 --max-time 10 --retry 99 --retry-delay 0"

tmp=/tmp/$name.setup

down() {

  site="https://$1"
  ver=$($CURL $site/v/$name)
  echo $ver

  rm -rf $tmp
  mkdir -p $tmp
  cd $tmp

  tar_name=$ARCH-$OS.tar
  out=$ver.$tar_name
  rm -rf $out
  $CURL $site/$name/$ver/$tar_name -o $out
  tar xvf $out
  rm -rf $out
  tar xvf $ver.txz -C $BIN
}

for i in ${MIRROR[@]}; do
  down $i && break
done

rm -rf $tmp
