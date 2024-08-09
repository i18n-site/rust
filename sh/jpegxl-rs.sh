#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*/*}
set -ex
cd $DIR

os_type=$(uname)

source ./sh/cflag.sh

if [ ! -d "jpegxl-rs" ]; then
  git clone --recursive --shallow-submodules --depth=1 \
    https://github.com/inflation/jpegxl-rs.git

  cd jpegxl-rs/jpegxl-src/libjxl
  ./deps.sh && cmake . -DBUILD_TESTING=OFF && make -j $(nproc) && make install
  rm CMakeCache.txt

  cd $DIR
fi
