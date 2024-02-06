use std::process::exit;

use clap::{arg, crate_version, Command};
use current_platform::CURRENT_PLATFORM;

pub fn cmdv(name: impl Into<clap::builder::Str>) -> Command {
  let cmd = Command::new(name)
    .disable_version_flag(true)
    .arg(arg!(-v --version ...))
    .arg(arg!(
        - -vv "version detail"
    ));

  let m = cmd.clone().ignore_errors(true).get_matches();

  let mut vv = m.get_one("vv") == Some(&true);

  if let Some(n) = m.get_one::<u8>("version") {
    let n = *n;
    if n > 0 {
      if n == 1 {
        println!(crate_version!());
        exit(0);
      } else {
        vv = true;
      }
    }
  }

  if vv {
    println!(
      r#"ver:{}
target:{}"#,
      crate_version!(),
      CURRENT_PLATFORM
    );
    exit(0);
  }

  cmd
}
