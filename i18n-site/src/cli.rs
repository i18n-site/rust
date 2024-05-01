use aok::{Result, OK};
use clap::arg;
use cmdv::cmdv;

use crate::run;

pub async fn cli() -> Result<()> {
  if let Some((m, _)) = cmdv!(
    arg!(-d --dir [PATH] "workdir"),
    arg!(-c --channel_ver [NAME] "root channel:version, default nightly:2022-01-01"),
    arg!(--s3 "upload to s3")
  ) {
    let channel_ver = m.get_one("channel_ver").map(|s: &String| s.into());
    let dir = m
      .get_one("dir")
      .map(|s: &String| s.into())
      .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| ".".into()));

    let s3 = m.get_one::<bool>("s3").cloned().unwrap_or(false);

    return run(dir, channel_ver, s3).await;
  }
  OK
}
