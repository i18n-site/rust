use std::{net::SocketAddr, sync::Arc};

use async_trait::async_trait;
use pingora_core::{
  server::Server,
  upstreams::peer::HttpPeer,
};
use pingora_load_balancing::{health_check::TcpHealthCheck, prelude::{LoadBalancer, RoundRobin}};
use pingora_proxy::{ProxyHttp, Session, http_proxy_service};


use crate::route::Upstream;

#[derive(Clone)]
pub struct MyProxy {
  load_balancer: Arc<LoadBalancer<RoundRobin>>,
}

#[async_trait]
impl ProxyHttp for MyProxy {
  type CTX = ();
  fn new_ctx(&self) -> Self::CTX {
    ()
  }

  async fn upstream_peer(
    &self,
    _session: &mut Session,
    _ctx: &mut Self::CTX,
  ) -> pingora_core::Result<Box<HttpPeer>> {
    let upstream = self
      .load_balancer
      .select(b"", 256)
      .ok_or_else(|| pingora_core::Error::new_str("No upstream found"))?;

    // 我们假设上游是 http
    let peer = HttpPeer::new(upstream.addr, false, "".to_string());
    Ok(Box::new(peer))
  }
}

pub fn srv(
  h1_addr: SocketAddr,
  upstream: Upstream,
) -> crate::Result<Server> {
  let mut my_server = Server::new(None)?;
  my_server.bootstrap();

  // 从 upstream 配置创建 upstreams
  let upstreams = upstream.addr_li.to_vec();

  // 创建一个轮询的负载均衡器
  let mut load_balancer = LoadBalancer::try_from_iter(upstreams)?;
  load_balancer.set_health_check(TcpHealthCheck::new());

  let proxy = MyProxy {
    load_balancer: Arc::new(load_balancer),
  };

  // H1 服务
  let h1_service = http_proxy_service(&my_server.configuration, proxy.clone());
  let mut service1 = pingora_core::services::listening::Service::new("H1 Proxy".to_string(), h1_service);
  service1.add_tcp(&h1_addr.to_string());
  my_server.add_service(service1);

  println!("H1 proxy listening on {h1_addr}");
  println!("Upstream: {:?}", upstream.addr_li);

  Ok(my_server)
}
