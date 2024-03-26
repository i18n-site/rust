use aok::{Result, OK};
use cli_tmpl::run;

#[tokio::main]
async fn main() -> Result<()> {
  i18n_bgu::boot(run).await?;
  OK
}
