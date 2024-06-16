#!/usr/bin/env bash

[ -z "${VER}" ] && export VER=$(cargo metadata --format-version=1 --no-deps | jq '.packages[] | .name + " " + .version' -r | grep "$(echo $PROJECT | sed 's/\./-/g') " | awk '{print $2}') || true
