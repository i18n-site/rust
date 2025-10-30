use std::borrow::Borrow;

pub use clap::{self, ArgAction, arg};
use clap::{ArgMatches, Command};
pub use const_str;
use current_platform::CURRENT_PLATFORM;

pub fn parse(
  project: impl Into<String>,
  ver: impl Borrow<[u64; 3]>,
  cmd_build: impl FnOnce(Command) -> Command,
) -> Option<ArgMatches> {
  let mut cmd = cmd_build(
    Command::new(project.into())
      .disable_version_flag(true)
      .disable_help_flag(true)
      .arg(arg!(-v --version "show version").action(clap::ArgAction::SetTrue))
      .arg(arg!(
        --vv "version detail"
      ))
      .arg(arg!(
        -h --help "print help"
      )),
  );
  {
    let cmd2 = cmd.clone();
    let m = cmd2.ignore_errors(true).get_matches();
    if let Some(help) = m.get_one::<bool>("help")
      && *help
    {
      xerr::log!(cmd.print_help());
      return None;
    }

    let vv = m.get_one("vv") == Some(&true);
    if let Some(n) = m.get_one("version")
      && *n
    {
      let ver = ver.borrow();
      println!("{}.{}.{}", ver[0], ver[1], ver[2]);
      return None;
    } else if vv {
      let ver = ver.borrow();
      println!(
        r#"ver:{}.{}.{}
target:{CURRENT_PLATFORM}"#,
        ver[0], ver[1], ver[2]
      );
      return None;
    }
  }

  let matches = cmd.get_matches();
  Some(matches)
}

#[cfg(feature = "macro")]
#[macro_export]
macro_rules! parse {
  ($cmd_build: expr) => {{
    $crate::parse(
      env!("CARGO_PKG_NAME"),
      [
        $crate::const_str::parse!(env!("CARGO_PKG_VERSION_MAJOR"), u64),
        $crate::const_str::parse!(env!("CARGO_PKG_VERSION_MINOR"), u64),
        $crate::const_str::parse!(env!("CARGO_PKG_VERSION_PATCH"), u64),
      ],
      $cmd_build,
    )
  }};
}
