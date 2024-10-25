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
  TARGET_LI=(aarch64-unknown-linux-gnu aarch64-unknown-linux-musl x86_64-unknown-linux-gnu x86_64-unknown-linux-musl)
  ;;
Darwin)
  TARGET_LI=$(rustc -vV | awk '/host/ { print $2 }')
  if ! command -v protoc &>/dev/null; then
    brew install protobuf
  fi
  ;;
esac

VER=$($DIR/VER.sh $PROJECT)

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

NAME=$(echo $PROJECT | sed 's/\./-/g')
build="cargo $build -p $NAME -Z build-std=std,panic_abort --release --target"

# echo $TARGET_LI | xargs -n1 -P$(nproc) $build
for target in ${TARGET_LI[@]}; do
  if [[ "$target" == *"aarch"* ]]; then
    feature="+neon"
  else
    feature="+sse2"
  fi
  RUSTFLAGS="$RUSTFLAGS -C target-feature=$feature" $build $target
done

if [[ "$unameOut" == MINGW* ]]; then
  wait
  # https://github.com/briansmith/ring/issues/1514
  target=aarch64-pc-windows-msvc
  TARGET_LI="$TARGET_LI $target"
  # Get Visual Studio installation directory
  VSINSTALLDIR=$(vswhere.exe -latest -requires Microsoft.VisualStudio.Component.VC.Llvm.Clang -property installationPath)/VC
  LLVM_ROOT=$VCINSTALLDIR/Tools/Llvm/x64
  export PATH=$PATH:/usr/local/bin/nasm:$LLVM_ROOT/bin
  ./target.sh $target
  $build $target
fi

for target in ${TARGET_LI[@]}; do
  $DIR/mv.sh $VER $target
done
