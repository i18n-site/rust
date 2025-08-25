use std::{future, net::SocketAddr, sync::Arc};

use faststr::FastStr;
use http_body_util::Full;
use i18n_gateway::{
  cert_loader::CertStrDb,
  conf::Conf,
  error::Result,
  route::{Protocol, Route, SiteConf, Upstream},
  srv,
};
use static_init::constructor;

pub const TEST_HOST: &str = "018007.xyz";
pub const H1_ADDR: &str = "127.0.0.1:9081";
pub const H2_ADDR: &str = "127.0.0.1:9082";
pub const H3_ADDR: &str = "127.0.0.1:9082";
pub const UPSTREAM_ADDR: &str = "127.0.0.1:9080";
pub const TEST_RESPONSE_BODY: &[u8] = b"Hello, from upstream!";

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

#[constructor(0)]
extern "C" fn start_server() {
  loginit::init();
  rustls::crypto::ring::default_provider()
    .install_default()
    .unwrap();
  std::thread::spawn(|| {
    let rt = tokio::runtime::Builder::new_multi_thread()
      .enable_all()
      .build()
      .unwrap();

    rt.block_on(async {
      // 启动上游服务
      tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(UPSTREAM_ADDR).await.unwrap();
        loop {
          let (stream, _) = listener.accept().await.unwrap();
          let io = hyper_util::rt::TokioIo::new(stream);
          tokio::task::spawn(async move {
            let service = hyper::service::service_fn(|_req| async {
              Ok::<_, hyper::Error>(hyper::Response::new(Full::new(
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

      // 配置 i18n_gateway 服务
      let db = MemCertDb;
      let host = "018007.xyz";
      let route = Route::default();
      let upstream = Upstream {
        addr_li: vec![UPSTREAM_ADDR.parse().unwrap()].into(),
        connect_timeout_sec: 1,
        request_timeout_sec: 1,
        max_retry: 1,
        protocol: Protocol::H1,
      };
      let site_conf = SiteConf {
        upstream: Arc::new(upstream),
        use_wildcast_cert: false,
      };
      route.host_conf.insert(host.into(), site_conf);

      let h1_addr: SocketAddr = H1_ADDR.parse().unwrap();
      let h2_addr: SocketAddr = H2_ADDR.parse().unwrap();
      let h3_addr: SocketAddr = H3_ADDR.parse().unwrap();
      dbg!(h1_addr, h2_addr, h3_addr);

      let conf = Conf {
        h1: h1_addr,
        h2: h2_addr,
        h3: h3_addr,
        route,
      };

      // 在后台运行 i18n_gateway 服务
      if let Err(e) = srv::run(conf, db).await {
        eprintln!("Failed to run server: {}", e);
      }
    });
  });
}
