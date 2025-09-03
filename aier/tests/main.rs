use aier::Aier;
use aok::{OK, Void};
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[tokio::test]
async fn test_async() -> Void {
  let aier = Aier::new("zxx", vec!["t1".into(), "t2".into(), "t3".into()]);

  aier.chat().await?;
  aier.chat().await?;
  aier.chat().await?;
  aier.chat().await?;
  aier.chat().await?;
  aier.chat().await?;
  OK
}
