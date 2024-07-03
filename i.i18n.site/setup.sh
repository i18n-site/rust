#!/usr/bin/env bash

if [ -z "$1" ]; then
  echo "USAGE : $0 APP"
  exit 1
else
  APP=$1
fi

CURL="curl --retry 3 -L --connect-timeout 6"

GH=/gh/i18n-site/dist@

HOST_LI=(
  "cdn.jsdmirror.com$GH"
  "xxai.eu.org/"
  "cdn.jsdelivr.net$GH"
  "codeberg.org/i18n-site/dist/raw/branch/"
  "fastly.jsdelivr.net$GH"
  "github.com/i18n-site/rust/releases/download/"
  "bitbucket.org/i18nsite/dist/raw/"
  "raw.githubusercontent.com/i18n-site/dist/"
)

if [ -t 0 ]; then
  line() {
    echo -ne "\033[4m$@\033[0m"
  }
  ered() {
    echo -ne "\033[31m$@\033[0m"
  }
  egray() {
    echo -en "\033[90m$@\033[0m"
  }
  egreen() {
    echo -ne "\033[32m$@\033[0m"
  }
  eyellow() {
    echo -ne "\033[93m$@\033[0m"
  }
  eblue() {
    echo -ne "\033[94m$@\033[0m"
  }
else
  line() {
    echo -en "$@"
  }
  ered() {
    line $@
  }
  egray() {
    line $@
  }
  egreen() {
    line $@
  }
  eyellow() {
    line $@
  }
  eblue() {
    line $@
  }
fi

throw() {
  ered "$@\n"
  exit 1
}

eurl() {
  echo "$(egray '>') $(line $(eblue $1))"
}

get_libc() {
  local os=$(uname -s)
  case $os in
  Darwin) echo "apple-darwin" ;;
  Linux)
    if ldd --version 2>&1 | grep -q 'musl'; then
      echo "unknown-linux-musl"
    else
      echo "unknown-linux-gnu"
    fi
    ;;
  CYGWIN* | MINGW* | MSYS*)
    echo "pc-windows-msvc"
    ;;
  *)
    throw unknown libc
    ;;
  esac
}

libc=$(get_libc)

get_arch() {
  if [[ "$libc" == "pc-windows-msvc" ]]; then
    local winarch=$(wmic os get osarchitecture | sed -n '2p')
    if [[ $winarch == "ARM 64"* ]]; then
      echo "aarch64"
    else
      echo "x86_64"
    fi
  else
    case $(uname -m) in
    aarch64 | arm64) echo "aarch64" ;;
    x86_64) echo "x86_64" ;;
    *)
      echo "unknown arch"
      ;;
    esac
  fi
}

arch=$(get_arch)

name="${arch}-${libc}"
HOST_LI_LEN=${#HOST_LI[@]}
start_idx=$((RANDOM % HOST_LI_LEN))

for ((i = 0; i < HOST_LI_LEN; i++)); do
  idx=$(((start_idx + i) % HOST_LI_LEN))
  prefix=${HOST_LI[$idx]}
  url="https://${prefix}v/${APP}"
  eurl $url
  ver=$($CURL -sS $url)
  if [[ -n "$ver" && "$ver" =~ ^[a-zA-Z0-9._-]+$ ]]; then
    if [[ $? -eq 0 ]]; then
      break
    fi
  else
    unset ver
  fi
done

if [ -z "$ver" ]; then
  echo "can't get version"
  exit 1
else
  echo "$APP version $ver"
fi

if command -v $APP &>/dev/null; then
  exe=$(which $APP)
  exever=$($exe -v)
  if [[ $? -eq 0 ]]; then
    if [[ "$ver" == "$exever" ]]; then
      egreen "$exe ( version $ver ) ALREADY INSTALLED\n"
      exit 0
    else
      sorted=$(printf "%s\n%s" "$exever" "$ver" | sort -V)
      first=$(echo "$sorted" | head -n1)
      if [[ $first == $ver ]]; then
        egreen "$exe ( version $exever > $ver )  ALREADY INSTALLED\n"
        exit 0
      fi
    fi
  fi
  rm -rf $exe || ered "CAN'T rm $exe\n"
fi

_TMP=$(mktemp -d)

onExit() {
  rm -rf $_TMP
}

trap onExit EXIT

TMP=$_TMP/$APP/$ver
mkdir -p $TMP
cd $TMP

auto_add_shells=("bash" "zsh")
current_shell=$(basename "$SHELL")

if [[ ! " ${auto_add_shells[@]} " =~ " ${current_shell} " ]]; then
  auto_add_shells+=("$current_shell")
fi

for ((i = 0; i < HOST_LI_LEN; i++)); do
  idx=$(((start_idx + i) % HOST_LI_LEN))
  prefix=${HOST_LI[$idx]}
  url="https://${prefix}${APP}/${ver}/${name}.tar"
  eurl $url

  $CURL -C - -OL "$url"
  if [[ $? -eq 0 ]]; then
    tar -xf "${name}.tar"
    mkdir -p o
    tar -xJf $ver.txz -C o

    if [ -z "$TO" ]; then
      TO=/usr/local/bin
      mkdir -p $TO
      if [ -w $TO ]; then
        TO=$TO
      else
        TO="$HOME/.bin"
      fi
    fi

    mkdir -p $TO

    if ! [ -w $TO ]; then
      throw "CAN'T WRITE $TO"
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

    mv -f o/* $TO/

    for file in "${exe_li[@]}"; do
      echo "$(egray +) $(egreen $TO/$file)"
    done

    bin="export PATH=$TO:\$PATH"

    if ! echo "$PATH" | grep -q "$TO"; then
      for shell in "${auto_add_shells[@]}"; do
        RC=.${shell}rc

        FILE="$HOME/$RC"

        if [ -f "$FILE" ]; then
          if ! grep -q "export PATH=$TO:\$PATH" "$FILE"; then
            echo -e "$bin\n$(cat $FILE)" >"$FILE"
            egray "added '$bin' → $FILE\n"
          fi
          if [[ "$shell" == "$current_shell" ]]; then
            echo -e "$(egreen PLEASE RUN:)\n  $(eyellow . \~/$RC)"
          fi
        else
          if [[ "$shell" == "$current_shell" ]]; then
            echo $PATH | grep -q "$TO" && continue
            echo "PLEASE ADD $(eyellow $bin) TO ENV PATH"
          fi
        fi
      done
    fi

    exit 0
  else
    echo "DOWNLOAD $(ered FAILED), TRY NEXT ..."
  fi
done

throw INSTALL FAILED
