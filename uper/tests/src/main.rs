use aok::{OK, Void};
use uper::{ArgMatches, Command};
use upgrade_host::UPGRADE_HOST;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

pub fn cmd_build(cmd: Command) -> Command {
  cmd
}

pub async fn run(matches: ArgMatches) -> Void {
  dbg!(matches);
  OK
}

#[tokio::main]
async fn main() -> Void {
  uper::load(
    UPGRADE_HOST,
    std::fs::read("/Users/z/host/conf/env/upgrade/pk")?
      .try_into()
      .unwrap(),
    cmd_build,
    run,
    "i18",
    [0, 1, 2],
  )
  .await
}
