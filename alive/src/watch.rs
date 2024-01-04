use aok::{Result, OK};
use enum_dispatch::enum_dispatch;

use crate::{
  db::{Kind, Watch},
  errlog, ok,
};

#[enum_dispatch]
pub trait Task {
  async fn ping<'a>(
    &self,
    kind: &'a Kind,
    watch: &'a Watch,
    host: &'a str,
    kind_arg: &'a str,
    watch_arg: &'a str,
  ) -> Result<()>;
}

pub async fn watch<'a>(
  kind: &'a Kind,
  watch: &'a Watch,
  host: &'a str,
  kind_arg: &'a str,
  watch_arg: &'a str,
  task: impl Task,
) -> Result<()> {
  match task.ping(kind, watch, host, kind_arg, watch_arg).await {
    Ok(_) => {
      // ok(kind, watch)
    }
    Err(err) => todo!(),
  }
  OK
}
