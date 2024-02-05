#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

mkdir -p dump/data

bun x mysql2rust -r src/m.rs

arg="-h$MYSQL_HOST -P$MYSQL_PORT -u$MYSQL_USER $MYSQL_DB"

ignore=(log state)
set +x
for table in $(mysql $arg -e 'show tables' | tail -n +2); do
  if ! [[ ${ignore[@]} =~ ${table} ]]; then
    out=dump/data/$table.sql
    mysqldump \
      --compression-algorithms=zstd \
      --set-gtid-purged=OFF \
      -t $arg $table >$out
    echo $table
    sed -i '/^-- Dump completed/d; /^-- MySQL dump/d' "$out"
  fi
done
