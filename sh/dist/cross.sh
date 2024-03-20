#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
ROOT=${DIR%/*/*}
set -ex

. project.sh

build="build" # -Z unstable-options

unameOut="$(uname -s)"

case "${unameOut}" in
MINGW*)
  choco install activeperl nasm &
  RUSTFLAGS="$RUSTFLAGS -C target-feature=+crt-static"
  TARGET_LI=$(rustc -vV | awk '/host/ { print $2 }')
  ;;
Linux)
  build="zigbuild"
  if ! command -v cargo-zigbuild &>/dev/null; then
    cargo install cargo-zigbuild
  fi
  TARGET_LI=$(rustup target list | awk '{print $1}' | grep "\-linux-" | grep -E "x86|aarch64" | grep -E "[musl|gun]$" | grep -v "i686-unknown-linux-musl")
  ;;
Darwin)
  TARGET_LI=$(rustc -vV | awk '/host/ { print $2 }')
  if ! command -v protoc &>/dev/null; then
    brew install protobuf
  fi
  ;;
esac

. $DIR/VER.sh

for target in ${TARGET_LI[@]}; do
  rm -rf $ROOT/target/$target/release
  ./target.sh $target &
done

wait

# if [ "$unameOut" == "Linux" ]; then
# docker pull i18nsite/x86_64-pc-windows-msvc-cross &
# docker pull i18nsite/aarch64-pc-windows-msvc-cross &
# fi

. RUSTFLAGS.sh
build="cargo $build -p $PROJECT -Z build-std=std,panic_abort --release --target"

# echo $TARGET_LI | xargs -n1 -P$(nproc) $build
for target in ${TARGET_LI[@]}; do
  $build $target
done

if [[ "$unameOut" == MINGW* ]]; then
  wait
  # https://github.com/briansmith/ring/issues/1514
  # target=aarch64-pc-windows-msvc
  # TARGET_LI="$TARGET_LI $target"
  # # Get Visual Studio installation directory
  # VSINSTALLDIR=$(vswhere.exe -latest -requires Microsoft.VisualStudio.Component.VC.Llvm.Clang -property installationPath)/VC
  # LLVM_ROOT=$VCINSTALLDIR/Tools/Llvm/x64
  # export PATH=$PATH:/usr/local/bin/nasm:$LLVM_ROOT/bin
  # ./target.sh $target
  # $build $target
fi

for target in ${TARGET_LI[@]}; do
  $DIR/mv.sh $VER $target
done
