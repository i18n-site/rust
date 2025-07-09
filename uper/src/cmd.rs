use clap::{ArgMatches, Command, arg};
use current_platform::CURRENT_PLATFORM;

pub struct Cmd {
  pub force_upgrade: bool,
}

pub fn parse(
  project: impl Into<String>,
  cmd_build: impl FnOnce(Command) -> Command,
  ver: &[u64; 3],
) -> Option<(Cmd, ArgMatches)> {
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
      ))
      .arg(arg!(--force_upgrade).action(clap::ArgAction::SetTrue)),
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
      println!("{}.{}.{}", ver[0], ver[1], ver[2]);
      return None;
    }
    if vv {
      println!(
        r#"ver:{}.{}.{}
target:{CURRENT_PLATFORM}"#,
        ver[0], ver[1], ver[2]
      );
      return None;
    }
  }

  let matches = cmd.get_matches();
  Some((
    Cmd {
      force_upgrade: matches.get_one("force_upgrade") == Some(&true),
    },
    matches,
  ))
}
