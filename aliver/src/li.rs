use alive::{api, status};

pub async fn post() -> re::msg!() {
  let r = status().await?;
  dbg!(&r);
  Ok::<api::StateLi, _>(r)
}
