use aok::{Result, OK};

#[tokio::main]
async fn main() -> Result<()> {
  i18n_bgu::boot(i18n_site::cli).await?;
  OK
}
