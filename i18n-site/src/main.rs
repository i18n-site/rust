use aok::{Result, OK};
use i18n_site::run;

#[tokio::main]
async fn main() -> Result<()> {
  i18n_bgu::boot(run).await?;
  OK
}
