use aok::{Result, OK};
use md_title::title_trim;
use static_init::constructor;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

// #[tokio::test]
// async fn test() -> Result<()> {
//   info!("{}", 123456);
//   OK
// }

#[test]
fn test() -> Result<()> {
  for i in [" # 123  ", "# 456    ", "756"] {
    info!(">{}<", title_trim(i));
  }
  OK
}
