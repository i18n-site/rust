use pgw::{NONE, Pg, Sql};
use static_init::dynamic;
use tokio::time;

// get postgres connection uri from environment
#[dynamic]
static PG: Pg = Pg::from_env("PG_URL");

// prepared sql
#[dynamic]
static SQL_NSPNAME: Sql = PG.sql("SELECT oid FROM pg_catalog.pg_namespace LIMIT 2");

use tokio_postgres::types::Oid;

#[tokio::test]
async fn main() -> anyhow::Result<()> {
  loginit::init();
  // dbg!(li().await?);
  for i in 0..2 {
    println!("loop {i}");
    match PG.q(&SQL_NSPNAME, &[]).await {
      Ok(li) => {
        for i in li {
          let oid: Oid = i.try_get(0).unwrap();
          dbg!(oid);
        }
      }
      Err(err) => {
        dbg!(err);
      }
    }
    let oid: Oid = PG
      .q00("SELECT oid FROM pg_catalog.pg_namespace LIMIT 1", NONE)
      .await?;
    dbg!(oid);
    time::sleep(std::time::Duration::from_secs(1)).await;
  }
  Ok(())
}
