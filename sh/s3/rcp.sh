#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}

set -e

PROJECT=$1

source $DIR/conf/S3.sh

keep10() {
  local REMOTE_PATH=$1/$PROJECT
  local FILES=$(rclone lsf --dirs-only $REMOTE_PATH | sort -V)
  local FILE_COUNT=$(echo "$FILES" | wc -l)
  local FILES_TO_DELETE=$((FILE_COUNT - 3))
  if [ $FILES_TO_DELETE -gt 0 ]; then
    echo "$FILES" | head -n $FILES_TO_DELETE | while read -r FILE; do
      rclone purge $REMOTE_PATH/$FILE
    done
  fi
}

sync() {
  local to=$1
  rclone copy . $to/ --exclude ".git/**"
  keep10 $to
}

for to in "${S3[@]}"; do
  sync $to &
done

wait
