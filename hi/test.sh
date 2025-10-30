#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -a
. ../../../hpc/conf/state/rust/smtp.env
. ../../../hpc/conf/state/rust/cfNotify.env
. ../../../hpc/conf/state/rust/hi.env
set +a
set -ex
cargo test -- --nocapture
