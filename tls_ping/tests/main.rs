use aok::{OK, Result};
use static_init::constructor;
use tls_ping::tls_ping;
// use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::test]
async fn test() -> Result<()> {
  tls_ping("6.cn", "123.57.242.213".parse()?, 30).await?;

  OK
}
