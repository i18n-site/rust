use alive::{api, status};

pub async fn post() -> re::msg!() {
  Ok::<api::StateLi, _>(status().await?)
}
