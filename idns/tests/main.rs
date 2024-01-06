use aok::{Result, OK};

#[tokio::test]
async fn test() -> Result<()> {
  let host = "mail.i18n.site";
  let r = idns::ip(host).await?;
  dbg!(r);
  OK
}
