use std::{net::SocketAddr, sync::Arc};

use http_body_util::BodyExt;
use hyper::{
  Request, Response,
  body::{Bytes, Incoming},
  server::conn::http2,
  service::service_fn,
};
use hyper_util::rt::TokioIo;
use rustls::ServerConfig;
use tokio::net::TcpListener;
use tokio_rustls::LazyConfigAcceptor;

use super::{proxy, util};
use crate::{
  cert_loader::{CertLoader, CertStrDb},
  error::{Error, Result},
  route::Route,
};

pub(super) async fn run<D: CertStrDb + std::fmt::Debug>(
  https_addr: SocketAddr,
  route: Arc<Route>,
  cert_loader: Arc<CertLoader<D>>,
) -> Result<()> {
  let listener = TcpListener::bind(https_addr).await?;

  loop {
    let (stream, _) = listener.accept().await?;
    let route = route.clone();
    let cert_loader = cert_loader.clone();

    tokio::task::spawn(async move {
      // 使用 LazyConfigAcceptor 进行动态证书加载
      let acceptor = LazyConfigAcceptor::new(rustls::server::Acceptor::default(), stream);

      match acceptor.await {
        Ok(start_handshake) => {
          let client_hello = start_handshake.client_hello();

          // 从 SNI 获取服务器名称
          let server_name = match client_hello.server_name() {
            Some(name) => name.to_string(),
            None => {
              eprintln!("No SNI provided in client hello");
              return;
            }
          };

          // 异步加载证书
          let config = match load_config_for_domain(&cert_loader, server_name.clone()).await {
            Ok(config) => config,
            Err(err) => {
              eprintln!("Failed to load certificate for {}: {:?}", server_name, err);
              return;
            }
          };

          // 完成 TLS 握手
          match start_handshake.into_stream(config).await {
            Ok(tls_stream) => {
              let io = TokioIo::new(tls_stream);

              if let Err(err) = http2::Builder::new(hyper_util::rt::TokioExecutor::new())
                .serve_connection(
                  io,
                  service_fn(move |req| handle_request(route.clone(), req)),
                )
                .await
              {
                eprintln!("h2 error: {:?}", err);
              }
            }
            Err(err) => {
              eprintln!("TLS handshake completion error: {:?}", err);
            }
          }
        }
        Err(err) => {
          eprintln!("TLS acceptor error: {:?}", err);
        }
      }
    });
  }
}

async fn handle_request(
  route: Arc<Route>,
  req: Request<Incoming>,
) -> Result<Response<http_body_util::combinators::BoxBody<Bytes, Error>>> {
  let host = util::extract_host(&req)?;

  if let Some(site_conf) = route.host_conf.get(&host) {
    // 转换请求
    let (method, path_and_query, headers) = util::hyper_to_reqwest(req).await?;

    let body = if method == GET { None } else { Some() };

    // 转发请求到上游服务器
    let res = proxy::proxy(method, &path_and_query, headers, body, &site_conf.upstream).await?;
    // 转换响应
    let hyper_res = util::reqwest_to_hyper(res).await?;
    Ok(hyper_res.map(|full| full.map_err(|e| -> Error { match e {} }).boxed()))
  } else {
    Ok(util::not_found_response().map(|full| full.map_err(|e| -> Error { match e {} }).boxed()))
  }
}

// 为指定域名加载 TLS 配置
async fn load_config_for_domain<D: CertStrDb + std::fmt::Debug>(
  cert_loader: &Arc<CertLoader<D>>,
  domain: String,
) -> Result<Arc<ServerConfig>> {
  // 尝试加载域名对应的证书
  match cert_loader.get(domain.clone()).await {
    Ok(Some(c)) => {
      let mut config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(c.rustls.cert.clone(), c.rustls.key.clone_key())
        .map_err(crate::error::Error::Rustls)?;
      config.alpn_protocols = vec![b"h2".to_vec()];

      Ok(Arc::new(config))
    }
    Ok(None) => {
      // 如果没有找到证书，可以返回默认证书或错误
      Err(Error::CertNotFound(domain))
    }
    Err(err) => {
      eprintln!("Error loading certificate for {}: {:?}", domain, err);
      Err(err)
    }
  }
}
