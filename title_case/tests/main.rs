use aok::{Result, OK};
use static_init::constructor;
use title_case::title_case;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::test]
async fn test() -> Result<()> {
  info!(
    "{}",
    title_case("what is i18n?".split(' '), "i18n 是什么？")
  );
  OK
}
