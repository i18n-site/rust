use aok::{Result, OK};

use crate::{
  db::{Kind, Watch},
  errlog, ok,
};

pub async fn watch(
  kind: &Kind,
  watch: &Watch,
  host: impl AsRef<str>,
  task: impl futures::Future<Output = Result<()>>,
) -> Result<()> {
  match task.await {
    Ok(_) => {
      // ok(kind, watch)
    }
    Err(err) => todo!(),
  }
  OK
}
