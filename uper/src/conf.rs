use aok::{OK, Result, Void};
use clap::{ArgMatches, Command};
use coarsetime::Clock;
use confdir::confdir;
use confer::FsConf;

use crate::{Uper, cmd, dns_check};

const PRE_CHECK: &str = "preCheck";

pub async fn load<F: 'static + Send + std::future::Future<Output = Result<()>>>(
  host_li: &[&str],
  pk: [u8; 32],
  cmd_build: impl FnOnce(Command) -> Command,
  run: impl FnOnce(ArgMatches) -> F,
  project: impl Into<String>,
  ver: [u64; 3],
) -> Void {
  let project = project.into();

  let (cmd, ing) = match cmd::parse(&project, cmd_build, &ver) {
    None => return OK,
    Some((cmd, matches)) => {
      let ing = tokio::spawn(run(matches));
      (cmd, ing)
    }
  };

  let confdir = confdir().join(&project);
  std::fs::create_dir_all(&confdir)?;
  let fs_conf = FsConf::new(confdir.join("upgrade.conf"));
  let mut conf = fs_conf.load().unwrap_or_default();

  let now_days = Clock::now_since_epoch().as_days();

  let check_upgrade = cmd.force_upgrade || {
    let pre_check: u64 = conf.get(PRE_CHECK, 0);
    let upgrade_freq: i64 = conf.get("freq", 7);
    if upgrade_freq < 0 {
      return ing.await?;
    }
    now_days > pre_check + (upgrade_freq as u64)
  };

  let channel: String = conf.get("channel", "stable".into());
  let txt_host_li = if check_upgrade {
    conf.set(PRE_CHECK, now_days);

    let host_prefix = format!("{project}-{channel}.");

    let mut txt_host_li = vec![];

    if let Some(host_li) = conf.str("host_li") {
      for i in host_li.split_whitespace() {
        txt_host_li.push(format!("{host_prefix}{i}"));
      }
    }

    if txt_host_li.is_empty() {
      txt_host_li = host_li
        .iter()
        .map(|i| format!("{host_prefix}{i}"))
        .collect();
    }

    txt_host_li
  } else {
    vec![]
  };

  xerr::log!(fs_conf.dump(&conf));

  if !txt_host_li.is_empty()
    && let Some(ver_url_li) = dns_check(&project, &ver, &txt_host_li).await?
  {
    let uper = Uper::load(&project, channel, ver_url_li).await;

    if let Ok(uper) = xerr::ok!(uper) {
      ing.await??;
      uper.join(pk).await?;
      return OK;
    }
  }
  ing.await?
}
