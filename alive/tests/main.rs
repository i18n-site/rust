/*

#[cfg(feature = "macro")]
mod test_macro {
}
*/

#[tokio::test]
async fn test() -> aok::Result<()> {
  loginit::init();
  alive::next().await?;
  Ok(())
}
