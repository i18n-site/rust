use aok::{OK, Void};
use static_init::constructor;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

genv::s!(PG_URL);

#[tokio::test]
async fn test() -> Void {
  let tls = tokio_postgres_tls::MakeRustlsConnect {
    config: tlsinit::CLIENT.clone(),
  };

  let (client, connection) = tokio_postgres::connect(&PG_URL, tls).await?;

  tokio::spawn(async move {
    if let Err(e) = connection.await {
      tracing::error!("connection error: {}", e);
    }
  });

  let rows = client.query("SELECT $1::TEXT", &[&"hi"]).await?;

  let value: &str = rows[0].get(0);

  info!("value: {:?}", value);

  assert_eq!(value, "hi");

  OK
}
