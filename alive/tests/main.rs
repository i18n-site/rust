/*

#[cfg(feature = "macro")]
mod test_macro {
}
*/

#[tokio::test]
async fn test() -> aok::Result<()> {
  loginit::init();
  // let li = alive::status().await?;
  use alive::Alive;

  let mut alive = Alive::new();
  alive.ping().await?;
  alive.ping().await?;
  Ok(())
}
