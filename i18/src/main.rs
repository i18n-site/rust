use aok::{Result, OK};
use i18::run;

#[tokio::main]
async fn main() -> Result<()> {
  i18n_bgu::boot(run).await?;
  OK
}
