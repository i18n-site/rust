use aok::{Result, OK};

#[tokio::test]
async fn test() -> Result<()> {
  for host in [
    "mail.i18n.site",
    "baidu.com",
    "youdao.com",
    "z.com",
    "a.com",
  ] {
    let r = idns::ip(host).await?;
    dbg!((host, r));
  }
  OK
}
