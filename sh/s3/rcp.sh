#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}

set -e

PROJECT=$1

source $DIR/conf/S3.sh

keep() {
  local REMOTE_PATH=$1/$PROJECT
  local DIRS=$(rclone lsf --dirs-only $REMOTE_PATH | sort -V)
  local DIR_COUNT=$(echo "$DIRS" | wc -l)
  local TO_DELETE=$((DIR_COUNT - 5))
  if [ $TO_DELETE -gt 0 ]; then
    echo "$DIRS" | head -n $TO_DELETE | while read -r d; do
      local fp=$REMOTE_PATH/$d
      # rclone lsf $fp | xargs -I {} rclone delete "$fp/{}"
      rclone purge $fp
    done
  fi
}

sync() {
  local to=$1
  rclone copy . $to/ --exclude ".git/**"
  keep $to
}

for to in "${S3[@]}"; do
  sync $to &
done

wait
