use aok::{Result, OK};
use static_init::constructor;
#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::test]
async fn test() -> Result<()> {
  // let r = lang_name_li([1, 2, 0], &Default::default();
  // info!("{:?}", r);
  OK
}
