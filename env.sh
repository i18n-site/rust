#!/usr/bin/env bash

export RUSTFLAGS='--cfg reqwest_unstable'
export RUST_LOG=debug,supervisor=warn,hyper=warn,rustls=warn,h2=warn,tower=warn

env_sh() {
  local nowdir=$(pwd)
  cd "$(dirname $(realpath ${BASH_SOURCE[0]}))"/../conf/conn
  local i
  for i in $@; do
    set -o allexport
    source "$i".sh
    set +o allexport
  done

  cd $nowdir
  unset -f env_sh
}

#env_sh host kv gt redis mq pg mail clip nchan api qdrant
