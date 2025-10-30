#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -e
set -a
. /etc/ops/ipv6_proxy/conf.sh
. /etc/ops/ipv6_proxy/ip_li.sh
set +a
set -x

cargo run -- $HOME/.config/aiapi/gemini.yml
