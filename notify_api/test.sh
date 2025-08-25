#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

set -a
. ../../../hpc/conf/state/env/_apiToken.env
. ../../../hpc/conf/state/env/denoNotifyApi.env
set +a

cargo test --all-features -- --nocapture
