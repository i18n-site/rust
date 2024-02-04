#!/usr/bin/env bash

set -ex
case "$(uname -s)" in
# MINGW*)
#   choco install activeperl nasm &
#   RUSTFLAGS="$RUSTFLAGS -C target-feature=+crt-static"
#   TARGET_LI=$(rustc -vV | awk '/host/ { print $2 }')
#   ;;
# Linux)
#   build="zigbuild"
#   if ! command -v cargo-zigbuild &>/dev/null; then
#     cargo install cargo-zigbuild
#   fi
#   TARGET_LI=$(rustup target list | awk '{print $1}' | grep "\-linux-" | grep -E "x86|aarch64" | grep -E "[musl|gun]$" | grep -v "i686-unknown-linux-musl")
#   ;;
Darwin)
  if ! command -v realpath &>/dev/null; then
    brew install coreutils || true
  fi
  ;;
esac
