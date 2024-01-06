use aok::{Result, OK};

#[tokio::test]
async fn test() -> Result<()> {
  use idns::{A, AAAA};
  let host = "mail.i18n.site";
  let li = A(host).await?;
  dbg!(host);
  OK
}
