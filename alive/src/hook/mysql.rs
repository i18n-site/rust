use aok::{Result, OK};

use crate::db::{Kind, Watch};

pub async fn ping(
  kind: &'a Kind,
  watch: &'a Watch,
  host: &'a str,
  kind_arg: &'a str,
  watch_arg: &'a str,
) -> Result<()> {
  OK
}
