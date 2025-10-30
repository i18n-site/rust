#![cfg_attr(docsrs, feature(doc_cfg))]

use log::info;
use nix::{
  sys::signal::{Signal, kill},
  unistd::Pid,
};

pub fn kill_port(port: u16) {
  let my_pid = std::process::id();
  if let Ok(processes) = listeners::get_processes_by_port(port) {
    for process in processes {
      if process.pid != my_pid {
        info!(
          "{} | SIGINT → {} {} on port {}",
          my_pid, process.pid, process.name, port
        );

        kill(Pid::from_raw(process.pid as i32), Signal::SIGINT).ok();
      }
    }
  }
}
