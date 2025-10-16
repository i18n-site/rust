use aok::{Void, OK};
use proxy_fetch::load;
use reqwest::{self, header::HeaderMap};

#[static_init::constructor(0)]
extern "C" fn _log_init() {
  log_init::init();

  rustls::crypto::aws_lc_rs::default_provider()
    .install_default()
    .unwrap();
}

genv::s!(PROXY_SUBSCRITION_URL: String);

#[tokio::test]
async fn test_proxy_fetch() -> Void {
  let fetch = load(PROXY_SUBSCRITION_URL.split(";")).await?;

  for _ in 0..100 {
    if let Ok(r) = xerr::ok!(
      fetch
        .run(
          reqwest::Method::GET,
          "https://ifconfig.me/ip",
          HeaderMap::new(),
          None::<&[u8]>,
        )
        .await
    ) {
      let txt = String::from_utf8_lossy(&r.body);

      dbg!(&txt);
    }
  }
  OK
}
