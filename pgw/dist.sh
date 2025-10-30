#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

mdi
# 提取版本号行
VERSION_LINE=$(grep "^version" Cargo.toml)

# 提取版本号
VERSION=$(echo $VERSION_LINE | cut -d '"' -f2)

# 分割版本号为主版本号.次版本号.修订号
MAJOR=$(echo $VERSION | cut -d '.' -f1)
MINOR=$(echo $VERSION | cut -d '.' -f2)
PATCH=$(echo $VERSION | cut -d '.' -f3)

# 修订号加1
PATCH=$((PATCH + 1))

# 拼接新的版本号
NEW_VERSION="$MAJOR.$MINOR.$PATCH"

# 替换版本号
sed -i "s/^version = \"$VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml
cargo build
git add -u
git commit -m "v$NEW_VERSION"
git pull
git push
cargo publish --registry crates-io
