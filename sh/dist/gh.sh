#!/usr/bin/env bash

set -ex

release=$1

shift

for i in "$@"; do
  gh release upload $release $i &
done
wait
