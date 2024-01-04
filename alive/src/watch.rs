use aok::{Result, OK};
use enum_dispatch::enum_dispatch;

use crate::{
  db::{Kind, Watch},
  errlog, ok,
};

#[enum_dispatch]
pub trait Task {
  async fn run(&self) -> Result<()>;
}

pub async fn watch(
  kind: &Kind,
  watch: &Watch,
  host: impl AsRef<str>,
  task: impl Task,
) -> Result<()> {
  match task.run().await {
    Ok(_) => {
      // ok(kind, watch)
    }
    Err(err) => todo!(),
  }
  OK
}
