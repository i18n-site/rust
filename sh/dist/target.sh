#!/usr/bin/env bash

set -ex

rustup target list --installed | grep -q $1 && exit 0
rustup target add $1
rustup update nightly
