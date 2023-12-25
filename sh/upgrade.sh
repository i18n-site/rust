#!/usr/bin/env bash

set -ex
export CARGO_REGISTRIES_CRATES_IO_PROTOCOL=git
cargo update
cargo upgrade -i --recursive --verbose
