use aok::{OK, Void};
use log::info;
use proxy_fetch::ProxyFetch;

#[static_init::constructor(0)]
extern "C" fn _log_init() {
  log_init::init();
}

genv::s!(SS_SUBSCRIPTION_URL: String);

#[tokio::test]
async fn test_proxy_fetch() -> Void {
  let proxy_fetch = ProxyFetch::load(SS_SUBSCRIPTION_URL.split(";")).await?;
  info!("{:?}", &proxy_fetch);
  OK
}
