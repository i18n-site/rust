use aok::{Result, OK};
use clap::arg;
use cmdv::cmdv;

use crate::run;

pub async fn cli() -> Result<()> {
  if let Some((m, _)) = cmdv!(
    arg!(-d --dir [PATH] "workdir"),
    arg!(-r --root_ver [NAME] "root version"),
    arg!(-s --s3 "upload to s3")
  ) {
    let ver = m.get_one("root_ver").map(|s: &String| s.into());
    let dir = m
      .get_one("dir")
      .map(|s: &String| s.into())
      .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| ".".into()));

    let s3 = m.get_one::<bool>("s3").cloned().unwrap_or(false);

    return run(dir, ver, s3).await;
  }
  OK
}
