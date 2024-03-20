#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}

set -e

source $DIR/conf/S3.sh

rclone copy $1 $S3/$(basename $1)
