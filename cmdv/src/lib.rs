use std::process::exit;

pub use clap;
use clap::{arg, Command};
use current_platform::CURRENT_PLATFORM;

pub fn cmdv(name: impl Into<clap::builder::Str>, ver: impl AsRef<str>) -> Command {
  let ver = ver.as_ref();
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
        println!("{ver}");
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
      ver, CURRENT_PLATFORM
    );
    exit(0);
  }

  cmd
}

#[macro_export]
macro_rules! cmdv {
  ($name:ident) => {
    $crate::cmdv(stringify!($name), $crate::clap::crate_version!())
  };
}
