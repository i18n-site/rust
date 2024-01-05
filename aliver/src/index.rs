pub async fn index() -> aerr::msg!() {
  Ok(alive::status().await?)
}
