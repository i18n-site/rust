#!/usr/bin/env zsh

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -e

setenv() {
  for i in "$@"; do
  eval "v=\"\$$i\""
  echo "$i=$v"
  direnv exec . flyctl secrets set $i="$v"
  done
}


setenv HEALTHCHECK LARK_BOT MYSQL_COMPRESS MYSQL_CONN_LIMIT MYSQL_DB MYSQL_HOST MYSQL_PORT MYSQL_PWD MYSQL_SSL MYSQL_USER NAME RUSTFLAGS RUST_BACKTRACE RUST_LOG SMTP_FROM SMTP_HOST SMTP_PASSWORD SMTP_USER TO_MAIL WXPUSH_ID WXPUSH_TOKEN
