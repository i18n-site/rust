use aok::{OK, Result};
use tokio::time::{Duration, sleep};
use tracing::info;

pub struct Client {}

impl Client {
  pub async fn test(&self) {
    info!("client test success");
  }
}

pub async fn connect() -> Result<Client> {
  info!("Sleeping for 3 seconds...");
  sleep(Duration::from_secs(3)).await;
  Ok(Client {})
}

static_::init!(CLIENT: Client {
  connect().await
});

#[tokio::main]
async fn main() -> Result<()> {
  static_::init().await?;
  info!("inited");
  CLIENT.test().await;
  OK
}
