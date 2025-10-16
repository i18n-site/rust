use aok::{Result, OK};

#[tokio::main]
async fn main() -> Result<()> {
  loginit::init();
  i18n_bgu::boot!("i18n.site", i18n_site::cli).await?;
  OK
}
