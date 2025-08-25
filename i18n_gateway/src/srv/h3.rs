use std::{net::SocketAddr, pin::Pin, sync::Arc};

use bytes::{Buf, Bytes};
use http::{Request, Response, StatusCode};
use http_body_util::{BodyExt, Empty};
use s2n_quic::{
  Server,
  provider::tls::s2n_tls::{
    callbacks::{ClientHelloCallback, ConfigResolver, ConnectionFuture},
    connection::Connection as S2nTlsConnection,
    error::Error as S2nError,
  },
};

use crate::{
  cert_loader::{CertLoader, CertStrDb},
  error::{Error, Result},
  route::Route,
};

/// 创建 H3 响应
fn create_h3_response(status: StatusCode) -> Response<()> {
  Response::builder()
    .status(status)
    .header("content-type", "text/plain")
    .body(())
    .unwrap_or_else(|_| {
      let mut res = Response::new(());
      *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
      res
    })
}

// 证书解析
struct CertResolver<D: CertStrDb> {
  cert_loader: Arc<CertLoader<D>>,
}

impl<D: CertStrDb> CertResolver<D> {
  fn new(cert_loader: Arc<CertLoader<D>>) -> Self {
    Self { cert_loader }
  }
}

impl<D: CertStrDb + std::fmt::Debug> ClientHelloCallback for CertResolver<D> {
  fn on_client_hello(
    &self,
    connection: &mut S2nTlsConnection,
  ) -> std::result::Result<Option<Pin<Box<dyn ConnectionFuture>>>, S2nError> {
    dbg!("on_client_hello called");
    let server_name = connection.server_name();
    dbg!(&server_name);
    let sni = match server_name {
      Some(name) => name.to_string(),
      None => {
        dbg!("SNI is missing");
        return Err(S2nError::application("Missing SNI".to_string().into()));
      }
    };
    dbg!(&sni);

    let cert_loader = self.cert_loader.clone();
    let fut = async move {
      dbg!("Fetching certificate for SNI:", &sni);
      let cert_entry = cert_loader
        .get(sni.clone())
        .await
        .map_err(|e| {
          dbg!("Error fetching certificate:", &e);
          S2nError::application(e.to_string().into())
        })?
        .ok_or_else(|| {
          dbg!("Certificate not found for SNI:", &sni);
          S2nError::application(Error::CertNotFound(sni).to_string().into())
        })?;

      dbg!("Certificate found, building TLS config");
      let config = s2n_quic::provider::tls::s2n_tls::Server::builder()
        .with_certificate(&cert_entry.pem.cert, &cert_entry.pem.key)?
        .with_application_protocols(&[b"h3".to_vec()])?
        .build()?;

      dbg!("TLS config built successfully");
      Ok(config.into())
    };

    dbg!("Returning connection future");
    Ok(Some(Box::pin(ConfigResolver::new(fut))))
  }
}

pub(super) async fn run<D: CertStrDb + std::fmt::Debug>(
  addr: SocketAddr,
  route: Arc<Route>,
  cert_loader: Arc<CertLoader<D>>,
) -> Result<()> {
  let tls = s2n_quic::provider::tls::s2n_tls::Server::builder()
    .with_application_protocols([b"h3"])?
    .with_client_hello_handler(CertResolver::new(cert_loader))?
    .build()
    .map_err(|e| Error::S2nQuicProvider(e.to_string()))?;

  let mut server = Server::builder()
    .with_tls(tls)?
    .with_io(addr)?
    .start()
    .map_err(|e| Error::S2nQuicProvider(e.to_string()))?;

  dbg!("H3 server started");

  while let Some(connection) = server.accept().await {
    dbg!("H3 connection accepted");
    let route = route.clone();
    tokio::spawn(async move {
      log_h3_error("connection", handle_connection(connection, route).await);
    });
  }

  Ok(())
}

async fn handle_connection(connection: s2n_quic::Connection, route: Arc<Route>) -> Result<()> {
  // Wrap the s2n-quic connection for h3
  let h3_conn = super::h3_conn::Connection::new(connection);
  let mut h3_server: h3::server::Connection<_, Bytes> = h3::server::builder()
    .build(h3_conn)
    .await
    .map_err(Error::H3Connection)?;

  loop {
    match h3_server.accept().await {
      Ok(Some(request_resolver)) => {
        dbg!("H3 request accepted");
        let route = route.clone();
        tokio::spawn(async move {
          log_h3_error(
            "request handling",
            handle_request(request_resolver, route).await,
          );
        });
      }
      Ok(None) => {
        dbg!("H3 request accepted None");
        // Connection closed
        break;
      }
      Err(e) => {
        dbg!(("H3 request accepted ", &e));
        log_h3_error("accept", Err(e) as std::result::Result<(), _>);
        break;
      }
    }
  }

  Ok(())
}

async fn handle_request(
  request_resolver: h3::server::RequestResolver<super::h3_conn::Connection, Bytes>,
  route: Arc<Route>,
) -> Result<()> {
  // 解析请求
  let (request, mut request_stream) = request_resolver
    .resolve_request()
    .await
    .map_err(Error::H3Stream)?;

  // 从请求头中提取主机
  let host = extract_host_from_h3_request(&request)?;

  // 检查此主机是否有路由
  if let Some(site_conf) = route.host_conf.get(&host) {
    // 收集请求体
    let (parts, _) = request.into_parts();
    let mut body_bytes = Vec::new();
    while let Some(chunk) = request_stream.recv_data().await.map_err(Error::H3Stream)? {
      body_bytes.extend_from_slice(chunk.chunk());
    }
    let body = Bytes::from(body_bytes);
    let req = Request::from_parts(parts, Empty::new().map_err(|e| match e {}).boxed());

    // 转换请求
    let (method, path_and_query, headers, _) = super::util::hyper_to_reqwest(req).await?;

    // 代理请求到上游服务器
    match super::proxy::proxy(method, &path_and_query, headers, Some(body), &site_conf.upstream).await {
      Ok(response) => {
        // 通过 H3 流发送响应
        let status = response.status();
        let headers = response.headers().clone();
        let body = response.bytes().await?;

        let mut res_for_h3 = Response::new(());
        *res_for_h3.status_mut() = status;
        *res_for_h3.headers_mut() = headers;

        match request_stream.send_response(res_for_h3).await {
          Ok(_) => {
            if !body.is_empty() {
              log_h3_error(
                "send response body",
                request_stream.send_data(body).await,
              );
            }
            log_h3_error("finish response stream", request_stream.finish().await);
          }
          Err(e) => {
            log_h3_error::<(), _>("send response headers", Err(e));
          }
        }
      }
      Err(e) => {
        log_h3_error("proxy request", Err(e) as std::result::Result<(), _>);
        // 发送错误响应
        let error_response = create_h3_response(StatusCode::INTERNAL_SERVER_ERROR);
        log_h3_error(
          "send error response",
          request_stream.send_response(error_response).await,
        );
      }
    }
  } else {
    // 发送 404 响应
    let not_found_response = create_h3_response(StatusCode::NOT_FOUND);
    log_h3_error(
      "send 404 response",
      request_stream.send_response(not_found_response).await,
    );
  }

  Ok(())
}

/// 从 H3 请求头中提取主机
fn extract_host_from_h3_request(req: &http::Request<()>) -> Result<faststr::FastStr> {
  use http::header::HOST;

  let host = req
    .headers()
    .get(HOST)
    .and_then(|h| h.to_str().ok())
    .or_else(|| req.uri().host())
    .ok_or(Error::NoHost)?;

  // 移除端口号
  let host = host.split(':').next().unwrap_or(host);

  Ok(faststr::FastStr::new(host))
}

/// 统一记 H3 错误, 如果 result 是 Err
fn log_h3_error<T, E: std::fmt::Debug>(context: &str, result: std::result::Result<T, E>) {
  if let Err(e) = result {
    eprintln!("H3 {} error: {:?}", context, e);
  }
}
