#!/usr/bin/env bash

echo $(cargo metadata --format-version=1 --no-deps | jq '.packages[] | .name + " " + .version' -r | grep "$(echo $1 | sed 's/\./-/g') " | awk '{print $2}') || true
