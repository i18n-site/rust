use aok::{Result, OK};

#[tokio::test]
async fn test() -> Result<()> {
  use idns::ping::use_ipv6;

  let use_ipv6 = use_ipv6().await;
  dbg!(use_ipv6);
  // for host in [
  //   "mail.i18n.site",
  //   "baidu.com",
  //   "youdao.com",
  //   "z.com",
  //   "google.com",
  // ] {
  //   let r = idns::A(host).await?;
  //   dbg!((host, r));
  // }
  OK
}
