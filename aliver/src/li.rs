use alive::{api, status};

pub async fn post() -> re::msg!() {
  let r = status().await?;
  Ok::<api::StateLi, _>(r)
}
