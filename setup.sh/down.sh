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

auto_add_shells=("bash" "zsh")
current_shell=$(basename "$SHELL")

if [[ ! " ${auto_add_shells[@]} " =~ " ${current_shell} " ]]; then
  auto_add_shells+=("$current_shell")
fi

for ((i = 0; i < host_li_len; i++)); do
  idx=$(((start_idx + i) % host_li_len))
  prefix=${host_li[$idx]}
  url="https://${prefix}${PROJECT}/${ver}/${name}.tar"
  echo "> $url"

  curl --retry 5 -C - -OL "$url"
  if [[ $? -eq 0 ]]; then
    tar -xf "${name}.tar"
    mkdir -p o
    tar -xJf $ver.txz -C o

    if mkdir -p /opt/bin; then
      BIN="/opt/bin"
    elif mkdir -p "$HOME/.bin"; then
      BIN="$HOME/.bin"
    else
      echo "can't create /opt/bin or $HOME/.bin"
      exit 1
    fi

    if command -v $PROJECT &>/dev/null; then
      rm -rf $(which $PROJECT)
    fi

    exe_li=()

    cd o
    for file in *; do
      # 检查文件是否具有可执行权限
      if [ -x "$file" ]; then
        # 将文件名添加到数组中
        exe_li+=("$file")
      fi
    done
    cd ..

    mv -f o/* $BIN/

    for file in "${exe_li[@]}"; do
      echo "+ $BIN/$file"
    done

    for shell in "${auto_add_shells[@]}"; do
      RC=.${shell}rc

      FILE="$HOME/$RC"

      if [ -f "$FILE" ]; then
        if [[ "$shell" == "$current_shell" ]]; then
          echo $PATH | grep -q "$BIN" && continue
        fi
        if ! grep -q "export PATH=$BIN:\$PATH" "$FILE"; then
          bin="export PATH=$BIN:\$PATH"
          echo -e "$bin\n$(cat $FILE)" >"$FILE"
          echo "added '$bin' → $FILE"
        fi
        if [[ "$shell" == "$current_shell" ]]; then
          echo -e "PLEASE RUN:\n  source ~/$RC"
        fi
      else
        if [[ "$shell" == "$current_shell" ]]; then
          echo $PATH | grep -q "$BIN" && continue
          echo "PLEASE ADD $BIN TO PATH"
        fi
      fi
    done

    exit 0
  else
    echo "$url download failed, try next ..."
  fi
done

echo "install failed !"
exit 1
