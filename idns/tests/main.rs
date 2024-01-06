use aok::{Result, OK};

#[tokio::test]
async fn test() -> Result<()> {
  for host in [
    "mail.i18n.site",
    "baidu.com",
    "youdao.com",
    "z.com",
    "google.com",
  ] {
    let r = idns::A(host).await?;
    dbg!((host, r));
  }
  OK
}
