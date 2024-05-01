use aok::{Result, OK};
use chrono::Local;
use clap::arg;
use cmdv::cmdv;

use crate::run;

pub async fn cli() -> Result<()> {
  if let Some((m, _)) = cmdv!(
    arg!(--channel [NAME] "channel, default nightly"),
    arg!(--dist_ver [VERSION] "dist version, default today"),
    arg!(-d --dir [PATH] "workdir"),
    arg!(--s3 "upload to s3")
  ) {
    let channel = m
      .get_one("channel")
      .map(|s: &String| s.into())
      .unwrap_or_else(|| "nightly".into());

    let ver = m
      .get_one("dist_ver")
      .map(|s: &String| s.into())
      .unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());

    let dir = m
      .get_one("dir")
      .map(|s: &String| s.into())
      .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| ".".into()));

    let s3 = m.get_one::<bool>("s3").cloned().unwrap_or(false);

    return run(channel, ver, dir, s3).await;
  }
  OK
}
