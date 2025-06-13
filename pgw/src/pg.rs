use std::{ops::Deref, sync::Arc};

use hidden_password::hidden_password;
use tokio::{
  sync::RwLock,
  time::{Duration, sleep},
};
use tokio_postgres::{
  self, Client, Error, NoTls, Row, Statement, ToStatement,
  error::SqlState,
  types::{FromSql, ToSql},
};

use crate::{IntoStatement, conn, into_statement::into_statement, tls::MakeRustlsConnect};

#[derive(Clone)]
pub struct Sql(pub(crate) Arc<_Sql>);

impl<T: Deref<Target = Sql>> IntoStatement<Statement> for &T {
  async fn into(self) -> Result<Statement, Error> {
    into_statement(self).await
  }
}

pub struct _Pg {
  pub uri: String,
  pub tls: bool,
  pub sql_li: Vec<Sql>,
  pub(crate) _client: Option<Client>,
}

pub type PgRw = Arc<RwLock<_Pg>>;

#[derive(Clone)]
pub struct Pg(PgRw);

pub(crate) fn is_close(err: &Error) -> bool {
  let code = err.code();
  code.is_none() || code == Some(&SqlState::ADMIN_SHUTDOWN) || err.is_closed()
}

pub(crate) fn err_code(err: &Error) -> &str {
  let err_code = err.code();
  match err_code {
    Some(code) => code.code(),
    None => "",
  }
}

macro_rules! client {
  ($self:ident, $body:ident) => {{
    let pg = &$self.0;
    loop {
      {
        let pg = &pg.read().await;
        if let Some(client) = &pg._client {
          loop {
            match $body!(client) {
              Ok(r) => return Ok(r),
              Err(err) => {
                let hidden_password_uri = hidden_password(&pg.uri);
                let code = err_code(&err);
                tracing::error!("❌ {hidden_password_uri} ERROR {code} : {err}");
                if is_close(&err) {
                  break;
                }
                return Err(err);
              }
            }
          }
        }
      }
      pg_conn(&pg).await?;
    }
  }};
}

async fn pg_conn(pg: &PgRw) -> Result<(), Error> {
  let mut _pg = pg.write().await;
  if _pg._client.is_some() {
    sleep(Duration::from_millis(100)).await;
    if _pg._client.is_some() {
      return Ok(());
    }
  }
  let uri = _pg.uri.clone();

  loop {
    macro_rules! conn {
      ($tls:expr) => {
        conn(&uri, $tls, pg.clone()).await?
      };
    }

    let client = if _pg.tls {
      conn!(MakeRustlsConnect::new(tlsinit::CLIENT.clone(),))
    } else {
      conn!(NoTls)
    };

    if client.is_some() {
      _pg._client = client;
      break;
    }
    sleep(Duration::from_secs(1)).await;
  }
  Ok(())
}

pub struct _Sql {
  pub(crate) sql: String,
  pub(crate) st: RwLock<Option<Statement>>,
  pub(crate) pg: Pg,
}

impl Pg {
  pub fn new(uri: impl Into<String>) -> Self {
    let uri = uri.into();
    let tls = if let Some(pos) = uri.find("?") {
      uri[pos + 1..].contains("sslmode=require")
    } else {
      false
    };

    Self(Arc::new(RwLock::new(_Pg {
      uri,
      tls,
      _client: None,
      sql_li: Vec::new(),
    })))
  }

  pub fn from_env(env: impl Into<String>) -> Self {
    let uri = std::env::var(env.into()).unwrap();
    Self::new(uri)
  }

  pub async fn q00<T: ToStatement, R>(
    &self,
    statement: impl IntoStatement<T>,
    params: &[&(dyn ToSql + Sync)],
  ) -> Result<R, Error>
  where
    for<'a> R: FromSql<'a>,
  {
    let statement = statement.into().await?;
    macro_rules! query_one {
      ($client:ident) => {{
        match $client.query_one(&statement, params).await {
          Ok(r) => Ok(r.get(0)),
          Err(err) => Err(err),
        }
      }};
    }
    client!(self, query_one)
  }

  pub async fn q0<T: ToStatement>(
    &self,
    statement: impl IntoStatement<T>,
    params: &[&(dyn ToSql + Sync)],
  ) -> Result<Row, Error> {
    let statement = statement.into().await?;
    macro_rules! query_one {
      ($client:ident) => {
        $client.query_one(&statement, params).await
      };
    }
    client!(self, query_one)
  }

  pub async fn q<T: ToStatement>(
    &self,
    statement: impl IntoStatement<T>,
    params: &[&(dyn ToSql + Sync)],
  ) -> Result<Vec<Row>, Error> {
    let statement = statement.into().await?;
    macro_rules! query {
      ($client:ident) => {
        $client.query(&statement, params).await
      };
    }
    client!(self, query)
  }

  pub async fn q00_<T: ToStatement, R>(
    &self,
    statement: impl IntoStatement<T>,
    params: &[&(dyn ToSql + Sync)],
  ) -> Result<Option<R>, Error>
  where
    for<'a> R: FromSql<'a>,
  {
    let statement = statement.into().await?;
    macro_rules! query_opt {
      ($client:ident) => {
        match $client.query_opt(&statement, params).await {
          Ok(r) => Ok(match r {
            Some(r) => match r.try_get(0) {
              Ok(r) => Some(r),
              Err(err) => return Err(err),
            },
            None => None,
          }),
          Err(err) => Err(err),
        }
      };
    }
    client!(self, query_opt)
  }

  pub async fn q0_<T: ToStatement>(
    &self,
    statement: impl IntoStatement<T>,
    params: &[&(dyn ToSql + Sync)],
  ) -> Result<Option<Row>, Error> {
    let statement = statement.into().await?;
    macro_rules! query_opt {
      ($client:ident) => {
        $client.query_opt(&statement, params).await
      };
    }
    client!(self, query_opt)
  }

  pub async fn e<T: ToStatement>(
    &self,
    statement: impl IntoStatement<T>,
    params: &[&(dyn ToSql + Sync)],
  ) -> Result<u64, Error> {
    let statement = statement.into().await?;
    macro_rules! execute {
      ($client:ident) => {
        $client.execute(&statement, params).await
      };
    }
    client!(self, execute)
  }

  pub async fn prepare(&self, query: impl AsRef<str>) -> Result<Statement, Error> {
    macro_rules! prepare {
      ($client:ident) => {
        $client.prepare(query.as_ref()).await
      };
    }

    // client(self, |c| c.prepare(query.as_ref())).await
    client!(self, prepare)
  }

  pub fn sql(&self, query: impl Into<String>) -> Sql {
    let sql = Sql(Arc::new(_Sql {
      sql: query.into(),
      st: RwLock::new(None),
      pg: self.clone(),
    }));
    let sql_clone = sql.clone();
    let me = self.clone();
    futures::executor::block_on(async move {
      me.0.write().await.sql_li.push(sql_clone);
    });
    sql
  }
}
