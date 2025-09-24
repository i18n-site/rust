#!/usr/bin/env bash

if ! [ -x "$(command -v upx)" ]; then
  nowdir=$(pwd)
  upx_version=$(curl -s https://api.github.com/repos/upx/upx/releases/latest | grep  '"tag_name":\s*"'|sed -E 's/.*"v([^"]+)".*/\1/')

  arch=$(uname -m)
  case $arch in
    x86_64)
      arch=amd64
      ;;
    aarch64)
      arch=arm64
      ;;
  esac
  cd /tmp
  upxdir=upx-$upx_version-${arch}_linux
  wget https://github.com/upx/upx/releases/download/v$upx_version/$upxdir.tar.xz -O upx.txz
  tar xfJ upx.txz
  exe=./$upxdir/upx
  chmod +x $exe
  mv $exe /usr/local/bin
  rm -rf $upxdir
  cd $nowdir
fi
