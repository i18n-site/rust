use std::{backtrace::Backtrace, future::Future, process::Command};

use aok::{Result, OK};
use ed25519_dalek::PUBLIC_KEY_LENGTH;

use crate::{Bgu, Conf, Ver};

genv::def! {
  DISABLE_AUTO_VER:i8 | 0;
}

pub const DISABLE_AUTO_VER_STR: &str = "DISABLE_AUTO_VER";

pub fn auto_ver(name: &str, now_ver: Ver) -> bool {
  if DISABLE_AUTO_VER() == 1 {
    return false;
  }
  let conf_ver = Into::<Conf>::into(name).ver();
  if let Some(ver) = conf_ver.get() {
    let ver: Ver = ver.into();
    if ver > now_ver {
      let path = crate::exe_path(name, &ver);
      let args: Vec<String> = std::env::args().skip(1).collect();

      let mut cmd = Command::new(path);
      cmd.args(args);
      for (key, value) in std::env::vars() {
        if key != DISABLE_AUTO_VER_STR {
          cmd.env(key, value);
        }
      }
      cmd.env(DISABLE_AUTO_VER_STR, "1");

      if let Ok(r) = xerr::ok!(cmd.spawn()) {
        if let Ok(r) = xerr::ok!(r.wait_with_output()) {
          std::process::exit(r.status.code().unwrap_or(0));
        }
      }
      return true;
    }
    conf_ver.rm();
  }
  false
}

pub async fn boot<F: Future<Output = Result<()>>>(
  pk: &[u8; PUBLIC_KEY_LENGTH],
  v_li: &[&str],
  down_li: &[&str],
  name: impl Into<String>,
  ver: [u64; 3],
  run: impl Fn() -> F,
) -> Result<()> {
  let ver = Ver(ver);
  let name = name.into();
  if auto_ver(&name, ver.clone()) {
    return OK;
  }
  let bgu = Bgu::new(pk, name, ver.clone(), v_li, down_li);
  if let Err(err) = run().await {
    eprintln!("{}\n❌ {}", Backtrace::capture(), err);
  }
  if let Some(bgu) = bgu {
    bgu.join().await?;
  }
  OK
}
