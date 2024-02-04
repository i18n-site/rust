#!/usr/bin/env bash

[ -z "${VER}" ] && export VER=$(cat $(dirname $(realpath $BASH_SOURCE))/../../$PROJECT/Cargo.toml | grep -E "^\s*version" | awk -F'"' '{print $2}') || true
