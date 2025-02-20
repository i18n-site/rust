#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -a
. ../../../hpc/state/conf/rust/smtp.env
. ../../../hpc/state/conf/rust/cfNotify.env
. ../../../hpc/state/conf/rust/hi.env
set +a
set -ex
cargo test -- --nocapture
