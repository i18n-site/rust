use tokio_postgres::types::ToSql;

use crate::Pg;

#[static_init::dynamic]
pub static PG: Pg = Pg::from_env("PG_URL");

pub struct Sqlify<T: ToSql + Sync>(pub T);

impl<T: ToSql + Sync> From<T> for Sqlify<T> {
  fn from(value: T) -> Self {
    Self(value)
  }
}

impl From<u64> for Sqlify<i64> {
  fn from(value: u64) -> Self {
    Self(value as i64)
  }
}

macro_rules! def {
  ($($name:ident),+) => {
$(
  #[macro_export]
  macro_rules! $name {
    ($$sql:expr) => {{
      use $crate::Sqlify;
      $crate::PG.$name($$sql, &[]).await
    }};
    ($$sql:expr, $$($$arg:expr),+) => {{
      use $crate::Sqlify;
      $crate::PG.$name($$sql, &[$$(
        &Into::<Sqlify<_>>::into($$arg).0
      ),+]).await
    }};
  }
)+
  };
}

def!(q, q0, q00, q0_, q00_);
