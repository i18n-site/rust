use aok::{Result, OK};
use hickory_proto::rr::RData;

#[tokio::test]
async fn test() -> Result<()> {
  use idns::{A, AAAA};
  let host = "mail.i18n.site";
  let r = A(host).await?;
  dbg!(&r);
  OK
}
