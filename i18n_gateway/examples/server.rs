use std::net::SocketAddr;

use i18n_gateway::{
  route::{Protocol, Upstream},
  srv,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // HTTP/1.1 代理地址
  let h1_addr: SocketAddr = "0.0.0.0:8080".parse()?;
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

  // 启动服务
  srv(h1_addr, upstream)?.run_forever();

  Ok(())
}
