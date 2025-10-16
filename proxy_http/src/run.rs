use std::{net::SocketAddr, sync::Arc};

use hyper_util::rt::TokioIo;
use log::error;
use proxy_fetch::Fetch;
use tokio::net::TcpListener;

use crate::{Result, handle};

pub async fn run(
  fetch: impl Into<Arc<Fetch>>,
  addr: SocketAddr,
  user: impl AsRef<str>,
  password: impl AsRef<str>,
) -> Result<()> {
  let fetch = fetch.into();
  let user = user.as_ref().to_string();
  let password = password.as_ref().to_string();

  let listener = TcpListener::bind(addr).await?;

  let user = Arc::new(user);
  let password = Arc::new(password);

  loop {
    let (stream, _) = listener.accept().await?;

    let io = TokioIo::new(stream);

    let fetch = Arc::clone(&fetch);
    let user = Arc::clone(&user);
    let password = Arc::clone(&password);

    tokio::task::spawn(async move {
      let service = hyper::service::service_fn(move |req| {
        handle(
          req,
          Arc::clone(&fetch),
          Arc::clone(&user),
          Arc::clone(&password),
        )
      });

      if let Err(err) = hyper::server::conn::http1::Builder::new()
        .serve_connection(io, service)
        .with_upgrades()
        .await
      {
        error!("Error serving connection: {:?}", err);
      }
    });
  }
}
