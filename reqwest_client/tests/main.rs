use aok::{OK, Void};
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[tokio::test]
async fn test_async() -> Void {
  let client = reqwest_client::proxy_iter()();
  let r = client.get("https://ifconfig.me/ip").send().await?;
  info!("{}", r.text().await?);

  OK
}
