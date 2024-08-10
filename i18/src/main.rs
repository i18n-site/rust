#![feature(let_chains)]

use aok::{Result, OK};
use clap::arg;
use cmdv::cmdv;
use i18::{find_i18n_dir_or_exit, i18n_conf_path, purge_arg, run, token, Conf};
use i18_conf::build_ignore;

pub async fn cli() -> Result<()> {
  if let Some((m, _)) = cmdv!(arg!(-d --workdir [path] "workdir"), purge_arg!(),) {
    let workdir = m
      .get_one("workdir")
      .map(|s: &String| s.into())
      .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| ".".into()));

    let purge = i18::purge::is(&m);

    // 递归翻译子目录
    for dir in find_i18n_dir_or_exit(&workdir) {
      println!("❯ {}", dir.display());
      let conf: Conf = yconf::load_or_exit(&i18n_conf_path(&dir));
      if purge {
        i18::purge::purge(&dir, &conf.i18n)?;
      } else {
        let token = token();
        run(&dir, &conf.i18n, &build_ignore(&conf.ignore), &token).await?;
      }
    }
  }

  OK
}

#[tokio::main]
async fn main() -> Result<()> {
  loginit::init();
  i18n_bgu::boot!(cli).await?;
  OK
}
