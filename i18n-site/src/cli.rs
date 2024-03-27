use aok::{Result, OK};
use clap::arg;
use cmdv::cmdv;

use crate::run;

pub async fn cli() -> Result<()> {
  if let Some((m, _)) = cmdv!(
    arg!(-d --dir [path] "workdir"),
    arg!(-u --upload [path] "upload")
  ) {
    let dir = m
      .get_one("dir")
      .map(|s: &String| s.into())
      .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| ".".into()));

    let upload = m.get_one::<bool>("upload").cloned().unwrap_or(false);

    return run(dir, upload).await;
  }
  OK
}
