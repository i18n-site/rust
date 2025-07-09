#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
ROOT=${ROOT%/*/*}
set -ex

. project.sh
VER=$(./VER.sh $PROJECT)

LOG=$ROOT/$PROJECT/log/$VER.md
if [ -f "$LOG" ]; then
  NOTE="-F $LOG"
else
  NOTE="-n $PROJECT/$VER"
fi

gh release create -d $PROJECT/$VER $NOTE
