#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

fp=id_ed25519

echo $ENC_PASSWD | gpg --batch --yes --passphrase-fd 0 --output $fp.enc --symmetric --cipher-algo AES256 $fp
