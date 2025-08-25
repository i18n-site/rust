#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

echo "启动测试后端服务器..."
echo "端口: 3000, 3001, 3002"
echo ""
echo "按 Ctrl+C 停止服务器"
echo ""

cargo run --example test_backend