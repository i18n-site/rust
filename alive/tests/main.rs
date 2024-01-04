/*

#[cfg(feature = "macro")]
mod test_macro {
}
*/

#[tokio::test]
async fn test() -> aok::Result<()> {
  loginit::init();
  let li = alive::status().await?;

  dbg!(li);
  // alive::next().await?;
  Ok(())
}
