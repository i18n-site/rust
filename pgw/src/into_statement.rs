pub use tokio_postgres::{self, Error, Statement, ToStatement};

use crate::Sql;

pub trait IntoStatement<T: ToStatement> {
  fn into(self) -> impl std::future::Future<Output = Result<T, Error>>;
}

impl<T: ToStatement> IntoStatement<T> for T {
  async fn into(self) -> Result<T, Error> {
    Ok(self)
  }
}

pub async fn into_statement(sql: &Sql) -> Result<Statement, Error> {
  let sql = &sql.0;
  loop {
    if let Some(st) = sql.st.read().await.as_ref() {
      return Ok(st.clone());
    }

    let st = sql.pg.prepare(&*sql.sql).await?;
    *sql.st.write().await = Some(st);
  }
}

impl IntoStatement<Statement> for Sql {
  async fn into(self) -> Result<Statement, Error> {
    into_statement(&self).await
  }
}
