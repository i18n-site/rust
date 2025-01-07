#!/usr/bin/env bash

# 检测未使用的依赖

if ! hash cargo-udeps 2>/dev/null; then
cargo install cargo-udeps --locked
fi

cargo +nightly udeps --workspace
