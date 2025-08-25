'''
use async_trait::async_trait;
use dashmap::DashMap;
use faststr::FastStr;
use i18n_gateway::{
  cert::Cert,
  cert_loader::{CertLoader, CertStrDb},
  error::Result,
};
use log::info;
use pingora::prelude::*;
use pingora_load_balancing::{health_check::TcpHealthCheck, selection::RoundRobin, LoadBalancer};
use pingora_proxy::{ProxyHttp, Session};
use rustls::server::ServerConfig;
use rustls_pki_types::ServerName;
use std::{future, net::SocketAddr, sync::Arc};

// --- 从原 examples/server.rs 借用的测试工具 ---

pub const TEST_RESPONSE_BODY: &[u8] = b"Hello World from Upstream";
pub const TEST_HOST: &str = "018007.xyz";
pub const UPSTREAM_ADDR: &str = "127.0.0.1:9080";
const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[derive(Debug)]
pub struct MemCertDb;

// 为测试实现一个内存证书数据库
impl CertStrDb for MemCertDb {
  fn get(
    &self,
    host: impl Into<FastStr>,
  ) -> impl future::Future<Output = Result<Option<(String, String)>>> + Send + Sync {
    let host_str: FastStr = host.into();
    async move {
      if host_str == TEST_HOST {
        // 从文件系统动态读取证书
        let cert_path = format!("{MANIFEST_DIR}/examples/ssl/{TEST_HOST}_ecc/fullchain.cer");
        let key_path = format!("{MANIFEST_DIR}/examples/ssl/{TEST_HOST}_ecc/{TEST_HOST}.key");

        let cert_pem = tokio::fs::read_to_string(&cert_path)
          .await
          .expect("无法读取证书文件");
        let key_pem = tokio::fs::read_to_string(&key_path)
          .await
          .expect("无法读取密钥文件");
        return Ok(Some((cert_pem, key_pem)));
      }
      Ok(None)
    }
  }
}

// 启动一个后台上游服务用于测试
pub fn bg_server() {
  tokio::spawn(async move {
    let listener = tokio::net::TcpListener::bind(UPSTREAM_ADDR).await.unwrap();
    loop {
      let (stream, _) = listener.accept().await.unwrap();
      let io = hyper_util::rt::TokioIo::new(stream);
      tokio::task::spawn(async move {
        let service =
          hyper::service::service_fn(|_req| async {
            Ok::<_, hyper::Error>(hyper::Response::new(http_body_util::Full::new(
              hyper::body::Bytes::from_static(TEST_RESPONSE_BODY),
            )))
          });
        if let Err(err) = hyper::server::conn::http1::Builder::new()
          .serve_connection(io, service)
          .await
        {
          eprintln!("Upstream server error: {}", err);
        }
      });
    }
  });
}

// --- Pingora 反向代理实现 ---

// MyTlsAccept 负责在 TLS 握手期间根据 SNI 按需提供证书
struct MyTlsAccept {
  cert_loader: Arc<CertLoader<MemCertDb>>,
  // 缓存 rustls ServerConfig 以提高性能
  cert_cache: DashMap<FastStr, Arc<ServerConfig>>,
}

impl MyTlsAccept {
  fn new(cert_loader: Arc<CertLoader<MemCertDb>>) -> Self {
    MyTlsAccept {
      cert_loader,
      cert_cache: DashMap::new(),
    }
  }

  async fn get_cert_from_loader(&self, name: &ServerName<'_>) -> Option<Arc<Cert>> {
    let host = match name {
      ServerName::DnsName(host) => host.as_ref().to_string(),
      _ => {
        // 不支持非 DNS 名称的 SNI
        return None;
      }
    };
    self.cert_loader.get(host).await.ok().flatten()
  }
}

#[async_trait]
impl TlsAccept for MyTlsAccept {
  fn name(&self) -> &str {
    "MyTlsAccept"
  }

  async fn get_cert(&self, name: &ServerName<'_>) -> Option<Arc<ServerConfig>> {
    let host: FastStr = match name {
      ServerName::DnsName(host) => host.as_ref().into(),
      _ => return None,
    };

    // 首先检查缓存
    if let Some(config) = self.cert_cache.get(&host) {
      return Some(config.clone());
    }

    // 如果缓存未命中，则从 CertLoader 加载
    if let Some(cert) = self.get_cert_from_loader(name).await {
      let rustls_cert = cert.rustls.cert.clone();
      let rustls_key = cert.rustls.key.clone_key();

      let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(rustls_cert, rustls_key)
        .ok()
        .map(Arc::new);

      if let Some(config) = &config {
        self.cert_cache.insert(host, config.clone());
      }
      return config;
    }

    None
  }
}

// LB (Load Balancer) 应用，负责代理和负载均衡
struct LB(Arc<LoadBalancer<RoundRobin>>);

#[async_trait]
impl ProxyHttp for LB {
  type CTX = ();
  fn new_ctx(&self) -> () {
    ()
  }

  async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
    let upstream = self
      .0
      .select(b"", 256) // hash is not used by RoundRobin
      .ok_or(Error::NoUpstreams)?;

    info!("Upstream selected: {:?}", upstream);

    let peer = Box::new(HttpPeer::new(upstream, true, TEST_HOST.into()));
    Ok(peer)
  }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // 初始化日志
  env_logger::init();

  // 启动上游服务
  bg_server();
  info!("Upstream server running at {}", UPSTREAM_ADDR);

  // 初始化 Pingora 服务器
  let mut my_server = Server::new(None)?;

  // 初始化证书加载器
  let cert_db = MemCertDb;
  let cert_loader = CertLoader::new(cert_db);

  // 定义上游服务和负载均衡器
  let upstreams = vec![UPSTREAM_ADDR.parse::<SocketAddr>().unwrap()];
  let mut lb = LoadBalancer::from_upstreams(upstreams);
  // 对上游服务进行健康检查
  let health_check = TcpHealthCheck::new();
  lb.set_health_check(health_check);
  lb.update_frequency = Some(std::time::Duration::from_secs(5));
  let lb = Arc::new(lb);


  // 创建并添加 HTTP -> HTTPS 跳转服务
  let mut http_service = Service::new("Redirect".into(), Arc::new(Redirect::new(301, true)));
  http_service.add_tcp("0.0.0.0:80");
  my_server.add_service(http_service);


  // 创建并添加反向代理服务
  let mut proxy_service = Service::new("Proxy".into(), Arc::new(LB(lb.clone())));
  let tls_accept = MyTlsAccept::new(cert_loader);
  proxy_service.add_tls_with_settings(
      "0.0.0.0:443",
      None,
      Arc::new(tls_accept)
  );
  my_server.add_service(proxy_service);


  info!("Starting Pingora server");
  my_server.bootstrap();
  // 启动负载均衡器的健康检查
  let hc_handle = lb.run_health_check();
  
  my_server.run_forever();

  // 等待健康检查任务结束
  hc_handle.await?;
  Ok(())
}
''