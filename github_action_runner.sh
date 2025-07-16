#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

# 到这里创建新的 runner
# https://github.com/organizations/i18n-site/settings/actions/runners/new?arch=arm64
# chown -R runner /Volumes/d/actions-runner
# 编辑 /etc/sudoers, 加上 runner ALL=(ALL) NOPASSWD:ALL

perms=$(ls -ld /opt/homebrew/ | awk '{print $1}')

# 检查是否具有组写权限
if [[ ${perms:5:1} != "w" ]]; then
  sudo chgrp -R admin /opt/homebrew/
  sudo chmod -R g+w /opt/homebrew/
  sudo dscl . append /Groups/admin GroupMembership runner
fi

export USER=runner
export HOME=/Users/$USER

cd /Volumes/d/actions-runner

exec sudo -u runner USER=$USER HOME=$HOME bash -c "exec ./run.sh"
