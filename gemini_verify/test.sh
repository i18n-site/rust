#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -e
set -a
. /etc/ipv6_proxy/env
. /etc/ipv6_proxy/host_li.env
set +a
set -x

cargo run -- $HOME/.config/aiapi/gemini.yml
