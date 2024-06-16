pub use clap;
use clap::{arg, Command};
use current_platform::CURRENT_PLATFORM;

pub fn cmdv(name: &'static str, ver: &str) -> Option<Command> {
  let cmd = Command::new(name)
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
  (
    $($arg:expr),*
    $(,)?
  ) => {{
    if let Some(mut cmd) = $crate::cmdv(env!("CARGO_PKG_NAME"),env!("CARGO_PKG_VERSION")) {
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
