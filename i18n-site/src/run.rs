use aok::{Result, OK};
use clap::arg;
use cmdv::cmdv;

pub async fn run() -> Result<()> {
  if let Some(cmd) = cmdv!() {
    let cmd = cmd.arg(arg!(-d --workdir [path] "workdir"));
    let m = cmd.get_matches();
    let workdir = m
      .get_one("workdir")
      .map(|s: &String| s.into())
      .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| ".".into()));

    dbg!(workdir);
  }
  OK
}
