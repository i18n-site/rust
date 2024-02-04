#!/usr/bin/env bash

set -ex

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

if ! [ -f "$HOME/.config/rclone/rclone.conf" ]; then
  if command -v rsync &>/dev/null; then
    rsync -av conf/ $HOME
  else
    cp -rT conf $HOME
  fi
fi
