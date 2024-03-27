use std::env;

pub use clap;
use clap::{arg, Command};
use current_platform::CURRENT_PLATFORM;

pub static NAME: &str = env!("CARGO_PKG_NAME");

pub static VER: &str = env!("CARGO_PKG_VERSION");

pub fn cmdv() -> Option<Command> {
  let cmd = Command::new(NAME)
    .disable_version_flag(true)
    .disable_help_flag(true)
    .arg(arg!(-v --version ...))
    .arg(arg!(
      - -vv "version detail"
    ))
    .arg(arg!(
      -h --help "print help"
    ));

  let m = cmd.clone().ignore_errors(true).get_matches();

  let mut vv = m.get_one("vv") == Some(&true);

  if let Some(n) = m.get_one::<u8>("version") {
    let n = *n;
    if n > 0 {
      if n == 1 {
        println!("{VER}");
        return None;
      }
      vv = true;
    }
  }

  if vv {
    println!(
      r#"ver:{}
target:{}"#,
      VER, CURRENT_PLATFORM
    );
    return None;
  }

  Some(cmd)
}

#[macro_export]
macro_rules! cmdv {
  (
    $($arg:expr),*
    $(,)?
  ) => {{
    if let Some(mut cmd) = $crate::cmdv() {
      let mut cmd = cmd$(.arg($arg))*;
      let m = cmd.clone().get_matches();
      if let Some(help) = m.get_one::<bool>("help") && *help {
        cmd.print_help()?;
        None
      }else{
        Some((m, cmd))
      }
    } else {
      None
    }
  }};
}
