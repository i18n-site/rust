use std::time::Duration;

use tokio_postgres::{Config, NoTls};
use tracing::error;

genv::def!(PG_URL);

pub struct Pg {
  pub client: tokio_postgres::Client,
  pub conn: tokio::task::JoinHandle<()>,
}

impl Pg {
  pub async fn from_env() -> Result<Self, tokio_postgres::Error> {
    let pg_url: String = PG_URL();
    let mut conf = pg_url.parse::<Config>()?;
    conf.connect_timeout(Duration::from_secs(9));

    let (client, conn) = conf.connect(NoTls).await?;

    Ok(Self {
      client,
      conn: tokio::spawn(async move {
        if let Err(e) = conn.await {
          error!("pg error : {}", e);
        }
      }),
    })
  }
}

impl Drop for Pg {
  fn drop(&mut self) {
    self.conn.abort();
  }
}

impl std::ops::Deref for Pg {
  type Target = tokio_postgres::Client;
  fn deref(&self) -> &Self::Target {
    &self.client
  }
}
