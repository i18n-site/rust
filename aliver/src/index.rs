pub async fn index() -> re::msg!() {
  Ok(alive::status().await?)
}
