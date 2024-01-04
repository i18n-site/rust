use aok::{Result, OK};

use crate::{
  db::{Kind, Watch},
  dberr,
};

pub async fn ping<'a>(
  kind: &'a Kind,
  watch: &'a Watch,
  host: &'a str,
  _: &'a str,
  _: &'a str,
) -> Result<()> {
  dbg!(host, watch.dns_type);

  OK
}
