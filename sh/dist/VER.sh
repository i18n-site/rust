#!/usr/bin/env bash
[ -z "${VER}" ] && export VER=$(cargo metadata --format-version=1 --no-deps | jq '.packages[] | .name + " " + .version' -r | grep "$PROJECT " | awk '{print $2}') || true
