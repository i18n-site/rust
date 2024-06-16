#!/usr/bin/env bash

if [ -z "$1" ]; then
  echo "USAGE : $0 PROJECT"
  exit 1
else
  export PROJECT=$1
fi

get_arch() {
  arch=$(uname -m)
  case $arch in
  aarch64 | arm64) echo "aarch64" ;;
  x86_64) echo "x86_64" ;;
  *)
    echo "unknown arch" && exit 1
    ;;
  esac
}

# Detect the system OS and libc
get_libc() {
  os=$(uname -s)
  case $os in
  Darwin) echo "apple-darwin" ;;
  Linux)
    if ldd --version 2>&1 | grep -q 'musl'; then
      echo "unknown-linux-musl"
    else
      echo "unknown-linux-gnu"
    fi
    ;;
  MINGW* | MSYS*) echo "pc-windows-msvc" ;;
  *) echo "unknown libc" && exit 1 ;;
  esac
}

host_li=(
  "github.com/i18n-site/rust/releases/download/"
  "jsd.onmicrosoft.cn/gh/i18n-site/dist@"
  "cdn.jsdelivr.net/gh/i18n-site/dist@"
  "jsd.cdn.zzko.cn/gh/i18n-site/dist@"
  "fastly.jsdelivr.net/gh/i18n-site/dist@"
  "codeberg.org/i18n-site/dist/raw/branch/"
  "xxai.eu.org/"
  "raw.githubusercontent.com/i18n-site/dist/"
  "bitbucket.org/i18nsite/dist/raw/"
)

arch=$(get_arch)
libc=$(get_libc)
name="${arch}-${libc}"
host_li_len=${#host_li[@]}
start_idx=$((RANDOM % host_li_len))

for ((i = 0; i < host_li_len; i++)); do
  idx=$(((start_idx + i) % host_li_len))
  prefix=${host_li[$idx]}
  url="https://${prefix}v/${PROJECT}"
  echo "> $url"
  ver=$(curl -sSL $url)
  if [[ $? -eq 0 ]]; then
    break
  fi
done

if [ -z "$ver" ]; then
  echo "can't get version"
  exit 1
else
  echo "version $ver"
fi

_TMP=$(mktemp -d)

onExit() {
  rm -rf $_TMP
}

trap onExit EXIT

TMP=$_TMP/$PROJECT/$ver
mkdir -p $TMP
cd $TMP

for ((i = 0; i < host_li_len; i++)); do
  idx=$(((start_idx + i) % host_li_len))
  prefix=${host_li[$idx]}
  url="https://${prefix}${PROJECT}/${ver}/${name}.tar"
  echo "> $url"

  curl --retry 5 -C - -OL "$url"
  if [[ $? -eq 0 ]]; then
    tar -xf "${name}.tar"
    tar -xJvf $ver.txz
    ls -alh
    exit 0
  else
    echo "$url download failed, try next ..."
  fi
done

echo "download failed !"
exit 1
