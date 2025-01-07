use aok::{Result, OK};

#[tokio::main]
async fn main() -> Result<()> {
  i18n_bgu::boot!(cli_tmpl::cli).await?;
  OK
}
