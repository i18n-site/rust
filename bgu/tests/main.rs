mod mirror;
use aok::{Result, OK};
use bgu::PUBLIC_KEY_LENGTH;
use static_init::constructor;
use tokio::time::{sleep, Duration};

pub const PK: &[u8; PUBLIC_KEY_LENGTH] = include_bytes!("i18n.pk");

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

use bgu::{ver, Bgu};

#[tokio::test]
async fn test() -> Result<()> {
  let bgu = Bgu::new(PK, "i18", ver!(), mirror::MIRROR);
  sleep(Duration::from_secs(1)).await;
  let ver = bgu.join().await?;
  dbg!(ver);
  OK
}
