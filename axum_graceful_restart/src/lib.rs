#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]

use std::net::SocketAddr;

use tracing::info;
use aok::Result;
use axum::Router;
use nix::{
  sys::signal::{Signal, kill},
  unistd::Pid,
};
use socket2::{Domain, Socket, Type};
use tokio::signal::unix::{SignalKind, signal};

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
    sigterm.recv().await;
    info!("{} | SIGTERM recv", std::process::id());
  };

  let app = app.into_make_service();

  let pid = std::process::id();
  kill_old(pid, addr.port()).await?;

  info!("{pid} | listen {addr}");

  axum::serve(listener, app)
    .with_graceful_shutdown(shutdown)
    .await?;

  info!("{pid} | shutdown");
  Ok(())
}

async fn kill_old(my_pid: u32, port: u16) -> Result<()> {
  if let Ok(processes) = listeners::get_processes_by_port(port) {
    for process in processes {
      if process.pid != my_pid {
        info!(
          "{} | SIGTERM → {} {} on port {}",
          my_pid, process.pid, process.name, port
        );

        kill(Pid::from_raw(process.pid as i32), Signal::SIGTERM).ok();
      }
    }
  }

  Ok(())
}
