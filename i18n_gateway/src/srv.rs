use std::{net::SocketAddr, sync::Arc};

use async_trait::async_trait;
use pingora_core::{
  listeners::TlsSettings,
  prelude::{LoadBalancer, RoundRobin},
  server::Server,
  services::listening::Service,
  upstreams::peer::HttpPeer,
};
use pingora_proxy::{ProxyHttp, Session};
use rustls::{
  server::{ClientHello, ResolvesServerCert},
  sign::{CertifiedKey, any_ecdsa_type},
};

use crate::{
  cert_loader::{CertLoader, CertStrDb},
  error::{Error, Result},
  route::Upstream,
};

struct MyResolver<D: CertStrDb> {
  cert_loader: Arc<CertLoader<D>>,
}

impl<D: CertStrDb> ResolvesServerCert for MyResolver<D> {
  fn resolve(&self, client_hello: ClientHello) -> Option<Arc<CertifiedKey>> {
    // 从 client_hello 中获取 SNI
    let sni = client_hello.server_name()?;
    dbg!("Resolving cert for SNI: {}", sni);

    // `resolve` 是同步的, 但是 `cert_loader.get` 是异步的.
    // 我们需要在 tokio 运行时上下文中阻塞等待 future.
    let handle = tokio::runtime::Handle::current();
    let cert_result = handle.block_on(self.cert_loader.get(sni.into()));

    match cert_result {
      Ok(Some(cert)) => {
        let cert_der: Vec<_> = cert.rustls.cert.iter().map(|c| c.clone().into()).collect();
        let key_der = &cert.rustls.key;
        // 使用 ecdsa 类型的 key
        let signing_key = any_ecdsa_type(key_der).ok()?;
        let key = CertifiedKey::new(cert_der, signing_key);
        Some(Arc::new(key))
      }
      Ok(None) => {
        dbg!("Cert not found for SNI: {}", sni);
        None
      }
      Err(e) => {
        dbg!("Error getting cert for SNI: {}, error: {}", sni, e);
        None
      }
    }
  }
}

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
    let upstream = self.load_balancer.select(b"", 256).ok_or_else(|| {
      pingora_core::Error::new_str(pingora_core::ErrorType::InternalError, "No upstream found")
    })?;

    // 我们假设上游是 http
    let peer = HttpPeer::new(upstream.addr, false, "".to_string());
    Ok(Box::new(peer))
  }
}

pub fn srv<D: CertStrDb>(
  h1_addr: SocketAddr,
  h2_addr: SocketAddr,
  upstream: Upstream,
  cert_db: D,
) -> Result<()> {
  let mut my_server = Server::new(None).map_err(|e| Error::Internal(e.to_string()))?;
  my_server.bootstrap();

  // 从 upstream 配置创建 upstreams
  let upstreams: Vec<_> = upstream
    .addr_li
    .iter()
    .map(|addr| HttpPeer::new(*addr, false, "".to_string()))
    .collect();

  // 创建一个轮询的负载均衡器
  let load_balancer = Arc::new(LoadBalancer::from_upstreams_unhealthy(upstreams));

  let proxy = MyProxy {
    load_balancer: load_balancer.clone(),
  };

  // H1 服务
  let mut h1_service = Service::new("H1 Proxy".to_string(), proxy.clone());
  let h1_addr_str = h1_addr.to_string();
  h1_service.add_tcp(&h1_addr_str);
  my_server.add_service(h1_service);

  // H2 服务 (TLS)
  let cert_loader = CertLoader::new(cert_db);
  let resolver = MyResolver { cert_loader };

  let mut tls_config = rustls::ServerConfig::builder()
    .with_safe_defaults()
    .with_no_client_auth()
    .with_cert_resolver(Arc::new(resolver));

  // 开启 h2
  tls_config.alpn_protocols = vec![b"h2".to_vec()];

  let tls_settings = TlsSettings::with_server_config(Arc::new(tls_config));

  let mut h2_service = Service::new("H2 Proxy".to_string(), proxy);
  let h2_addr_str = h2_addr.to_string();
  h2_service.add_tls_with_settings(&h2_addr_str, Some(tls_settings));
  my_server.add_service(h2_service);

  println!("H1 proxy listening on {h1_addr}");
  println!("H2 proxy listening on {h2_addr}");
  println!("Upstream: {:?}", upstream.addr_li);

  my_server.run_forever();

  Ok(())
}
