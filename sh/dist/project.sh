#!/usr/bin/env bash

if [ -z "$1" ]; then
  echo "USAGE : $0 project_name"
  exit 1
else
  export PROJECT=$1
fi
