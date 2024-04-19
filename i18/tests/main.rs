use aok::{Result, OK};
use i18::lang_name_li;
use static_init::constructor;
use tracing::info;
#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::test]
async fn test() -> Result<()> {
  let r = lang_name_li([1, 2, 0], &Default::default()).await?;
  info!("{:?}", r);
  OK
}
