#!/usr/bin/env bash

set -ex

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

if ! [ -f "$HOME/.config/rclone/rclone.conf" ]; then
  rsync --exclude=.git -av conf/ $HOME
fi
