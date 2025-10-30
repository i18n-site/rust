#![cfg_attr(docsrs, feature(doc_cfg))]

use std::net::SocketAddr;

use aok::Result;
use axum::Router;
use kill_port::kill_port;
use socket2::{Domain, Socket, Type};
use tokio::signal::unix::{SignalKind, signal};
use tracing::info;

fn listen(addr: SocketAddr) -> Result<tokio::net::TcpListener> {
  let socket = Socket::new(Domain::for_address(addr), Type::STREAM, None)?;
  socket.set_reuse_port(true)?;
  socket.bind(&addr.into())?;
  socket.listen(128)?;
  let std_listener: std::net::TcpListener = socket.into();
  std_listener.set_nonblocking(true)?;
  let tokio_listener = tokio::net::TcpListener::from_std(std_listener)?;
  Ok(tokio_listener)
}

pub async fn serve(addr: SocketAddr, app: Router) -> Result<()> {
  let listener = listen(addr)?;

  let shutdown = async {
    let mut sigterm = signal(SignalKind::terminate()).expect("create SIGTERM failed");
    let mut sigint = signal(SignalKind::interrupt()).expect("create SIGINT failed");
    tokio::select! {
        _ = sigterm.recv() => {
            info!("{} | SIGTERM recv", std::process::id());
        }
        _ = sigint.recv() => {
            info!("{} | SIGINT (Ctrl+C) recv", std::process::id());
        }
    }
  };

  let app = app.into_make_service();

  let pid = std::process::id();
  kill_port(addr.port());

  info!("{pid} | listen {addr}");

  axum::serve(listener, app)
    .with_graceful_shutdown(shutdown)
    .await?;

  info!("{pid} | shutdown");
  Ok(())
}
