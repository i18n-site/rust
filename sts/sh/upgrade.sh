#!/usr/bin/env bash

set -ex
cargo update
cargo upgrade --incompatible
