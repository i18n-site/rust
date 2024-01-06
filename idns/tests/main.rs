use aok::{Result, OK};

#[tokio::test]
async fn test() -> Result<()> {
  use idns::{A, AAAA};
  let host = "mail.i18n.site";
  let r = A(host).await?;
  dbg!(&r);
  // let r = AAAA(host).await?;
  // dbg!(&r);
  OK
}
