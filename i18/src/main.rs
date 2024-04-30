#![feature(let_chains)]

use aok::{Result, OK};
use clap::arg;
use cmdv::cmdv;
use i18::{conf, run, token};

pub async fn cli() -> Result<()> {
  if let Some((m, _)) = cmdv!(arg!(-d --workdir [path] "workdir")) {
    let workdir = m
      .get_one("workdir")
      .map(|s: &String| s.into())
      .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| ".".into()));
    run(&workdir, &conf(&workdir)?.into(), token()).await?;
  }

  OK
}

#[tokio::main]
async fn main() -> Result<()> {
  loginit::init();
  i18n_bgu::boot!(cli).await?;
  OK
}
