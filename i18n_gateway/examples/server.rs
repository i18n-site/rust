use std::net::SocketAddr;

use i18n_gateway::{
  route::{Protocol, Upstream},
  srv,
};

// 引入我们刚刚创建的证书加载模块
mod cert_db;
use cert_db::FileCertDb;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // HTTP/1.1 代理地址
  let h1_addr: SocketAddr = "0.0.0.0:8080".parse()?;
  // HTTP/2 代理地址 (TLS)
  let h2_addr: SocketAddr = "0.0.0.0:8443".parse()?;
  // 上游服务地址
  let upstream_addr: SocketAddr = "127.0.0.1:9000".parse()?;

  // 配置上游服务
  let upstream = Upstream {
    addr_li: vec![upstream_addr].into_boxed_slice(),
    connect_timeout_sec: 3,
    request_timeout_sec: 10,
    max_retry: 3,
    protocol: Protocol::H1, // 假设上游是 H1
  };

  // 实例化文件证书加载器
  let cert_db = FileCertDb;

  // 启动服务
  srv(h1_addr, h2_addr, upstream, cert_db)?;

  Ok(())
}
