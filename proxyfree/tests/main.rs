#[static_init::constructor(0)]
extern "C" fn _log_init() {
  log_init::init();
}

#[tokio::test]
async fn test_proxyscrape() -> anyhow::Result<()> {
  let proxies = proxyfree::fetch::proxyscrape().await?;
  assert!(!proxies.is_empty());
  println!("fetched {} proxies", proxies.len());

  Ok(())
}
