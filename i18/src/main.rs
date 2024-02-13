use cmdv::cmdv;

mod mirror;
use aok::{Result, OK};
use bgu::{boot, ver, PUBLIC_KEY_LENGTH};
use static_init::constructor;

pub const PK: &[u8; PUBLIC_KEY_LENGTH] = include_bytes!("i18n.pk");

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

// async fn main() -> Result<()> {
//   dbg!("test main");
//   OK
// }

pub async fn run() -> Result<()> {
  if let Some(cmd) = cmdv!(i18) {
    dbg!(cmd);
  }
  // .arg(arg!(-k --key <key> "key file path").required(false))
  // .arg(arg!(-c --create "create key if not exist"))
  // .arg(arg!([fp] "file path"));

  // let m = cmd.clone().get_matches();
  OK
}

#[tokio::main]
async fn main() -> Result<()> {
  boot(PK, "i18", ver!(), mirror::MIRROR, run).await?;
  OK
}
