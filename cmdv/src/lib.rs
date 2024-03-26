use std::env;

pub use clap;
use clap::{arg, Command};
use current_platform::CURRENT_PLATFORM;

pub static NAME: &str = env!("CARGO_PKG_NAME");

pub fn cmdv(ver: impl AsRef<str>) -> Option<Command> {
  let ver = ver.as_ref();
  let cmd = Command::new(NAME)
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
        println!("{ver}");
        return None;
      }
      vv = true;
    }
  }

  if vv {
    println!(
      r#"ver:{}
target:{}"#,
      ver, CURRENT_PLATFORM
    );
    return None;
  }

  Some(cmd)
}

#[macro_export]
macro_rules! cmdv {
  () => {
    $crate::cmdv($crate::clap::crate_version!())
  };
}
