use aok::{Result, OK};
use enum_dispatch::enum_dispatch;

use crate::{
  db::{Kind, Watch},
  errlog, ok,
};

#[enum_dispatch]
pub trait Task {
  async fn ping(
    &self,
    kind: &'a Kind,
    watch: &'a Watch,
    host: &'a str,
    kind_arg: &'a str,
    watch_arg: &'a str,
  ) -> Result<()>;
}

pub async fn watch(
  kind: &Kind,
  watch: &Watch,
  host: impl AsRef<str>,
  task: impl Task,
) -> Result<()> {
  match task.ping().await {
    Ok(_) => {
      // ok(kind, watch)
    }
    Err(err) => todo!(),
  }
  OK
}
