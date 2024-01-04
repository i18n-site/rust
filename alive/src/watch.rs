use aok::{Result, OK};

use crate::{errlog, ok};

pub async fn watch(task: impl futures::Future<Output = Result<()>>) -> Result<()> {
  // xerr::log!(
  //   async move {
  //     if let Err(err) = result {
  //       dbg!(&err);
  //     }
  //   }
  //   .await
  // );
  OK
}
