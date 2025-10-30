use aok::{OK, Void};
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[tokio::test]
async fn test_async() -> Void {
  let domain = "i18-stable.u-01.eu.org";

  let r = idoh::resolve(domain, "TXT", |li| {
    for i in li {
      tracing::info!("  {:?}", i);
      if i.r#type == idoh::record_type::TXT {
        return Ok(Some(i.data));
      }
    }
    Ok(None)
  })
  .await;

  if let Ok(txt) = xerr::ok!(r) {
    info!("{txt:?}");
  }

  OK
}
