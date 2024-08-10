#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

fp=id_ed25519

if [ ! -f "$HOME/.ssh/$fp" ]; then
  echo $ENC_PASSWD | gpg --batch --yes --passphrase-fd 0 --output $fp --decrypt $fp.enc
  mkdir -p ~/.ssh
  mv $fp ~/.ssh/
  chmod 700 ~/.ssh
  chmod 600 ~/.ssh/$fp
fi

if [ ! -d "conf" ]; then
  git clone --depth=1 git@github.com:i18n-pri/bin.conf.git conf
fi

./os.sh
