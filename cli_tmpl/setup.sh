#!/usr/bin/env bash

OS=$(uname | tr '[:upper:]' '[:lower:]')
ARCH=$(arch)
echo "${OS}-${ARCH}"

# windows-arm64
# windows-x86_64
# linux-arm64
# linux-x86_64
# darwin-arm64
# darwin-x86_64
