use tokio::time;
use tokio_postgres::{Client, Error, Socket, connect, tls::MakeTlsConnect};

use crate::pg::{PgRw, err_code, is_close};

pub async fn conn<T>(config: &str, tls: T, pg: PgRw) -> Result<Option<Client>, Error>
where
  T: MakeTlsConnect<Socket>,
  <T as MakeTlsConnect<Socket>>::Stream: std::marker::Send + 'static,
{
  match connect(config, tls).await {
    Ok((client, connection)) => {
      tokio::spawn(async move {
        if let Err(e) = connection.await {
          let code = err_code(&e);
          tracing::error!("❌ {code} → {e}");

          if is_close(&e) {
            let mut pg = pg.write().await;
            pg._client = None;
            for i in &mut pg.sql_li {
              *i.0.st.write().await = None;
            }
          }
        }
      });
      return Ok(Some(client));
    }
    Err(err) => {
      tracing::error!("❌ {err}");
      time::sleep(std::time::Duration::from_secs(1)).await;
    }
  }
  Ok(None)
}
