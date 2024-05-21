#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

# 到这里创建新的 runner
# https://github.com/organizations/i18n-site/settings/actions/runners/new?arch=arm64
# chown -R runner /Volumes/d/actions-runner

# 编辑 /etc/sudoers, 加上 runner ALL=(ALL) NOPASSWD:ALL

export USER=runner
export HOME=/Users/$USER

cd /Volumes/d/actions-runner

sudo chown -R runner /opt/homebrew/Library/Homebrew/
sudo chown -R runner /opt/homebrew/var/homebrew

exec sudo -u runner USER=$USER HOME=$HOME bash -c "exec ./run.sh"
