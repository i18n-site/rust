use std::time::Duration;

use anyhow::Result;
use log::info;
use nix::{
  sys::signal::{Signal, kill},
  unistd::Pid,
};
use tokio::time::{sleep, timeout};

#[static_init::constructor(0)]
extern "C" fn _log_init() {
  log_init::init();
}

#[tokio::test]
async fn test_restart_signal() -> Result<()> {
  let rx = restart_signal::restart_signal();
  let pid = Pid::from_raw(std::process::id() as i32);
  kill(pid, Signal::SIGQUIT)?;
  let result = timeout(Duration::from_secs(1), rx.recv()).await;
  match result {
    Ok(Ok(_)) => {
      // This is the expected outcome
    }
    Ok(Err(e)) => {
      panic!("channel received an error: {}", e);
    }
    Err(_) => {
      panic!("timeout waiting for signal");
    }
  }
  Ok(())
}
