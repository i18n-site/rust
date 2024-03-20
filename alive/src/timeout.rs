use std::time::Duration;

use aok::{Null, OK};
use futures::Future;

use crate::{
  db::{Kind, Watch},
  errlog,
};
pub const TIMEOUT: Duration = Duration::from_secs(180);

pub async fn timeout<'a>(
  kind: &'a Kind,
  host: &'a str,
  watch: &'a Watch,
  task: impl Future<Output = Null> + 'a,
) -> Null {
  match tokio::time::timeout(TIMEOUT, task).await {
    Err(_) => {
      xerr::log!(
        errlog(
          kind,
          host,
          watch,
          format!("运行超时 ( {} 分钟 )", TIMEOUT.as_secs() / 60),
          "",
        )
        .await,
      );
      OK
    }
    Ok(r) => r,
  }
}
