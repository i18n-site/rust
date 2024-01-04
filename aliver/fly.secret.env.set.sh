#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -e

setenv() {
  all=""
  for i in "$@"; do
    eval "v=\"\$$i\""
    all="$all $i=$v"
  done
  direnv exec . flyctl secrets set $all
}

setenv HEALTHCHECK LARK_BOT MYSQL_COMPRESS MYSQL_CONN_LIMIT MYSQL_DB MYSQL_HOST MYSQL_PORT MYSQL_PWD MYSQL_SSL MYSQL_USER NAME RUST_BACKTRACE RUST_LOG SMTP_FROM SMTP_HOST SMTP_PASSWORD SMTP_USER TO_MAIL WXPUSH_ID WXPUSH_TOKEN
