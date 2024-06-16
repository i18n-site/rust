#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

# 到这里创建新的 runner
# https://github.com/organizations/i18n-site/settings/actions/runners/new?arch=arm64
# chown -R runner /Volumes/d/actions-runner

# mac 需要给 /opt/homebrew/ 加权限
# sudo chgrp -R admin /opt/homebrew/
# sudo chmod -R g+w /opt/homebrew/
# sudo dscl . append /Groups/admin GroupMembership runner
# 最后编辑 /etc/sudoers, 加上 runner ALL=(ALL) NOPASSWD:ALL

export USER=runner
export HOME=/Users/$USER

cd /Volumes/d/actions-runner

sudo chown -R runner /opt/homebrew/Library/Homebrew/
sudo chown -R runner /opt/homebrew/var/homebrew

exec sudo -u runner USER=$USER HOME=$HOME bash -c "exec ./run.sh"
