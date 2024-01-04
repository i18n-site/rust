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

  match watch.dns_type {
    4 => {}
    6 => {}
    _ => {
      dberr!(
        DnsTypeNotSupported
        "watch_id={} host={} dns_type={}",
        watch.dns_type,
        host,
        watch.id
      );
      return OK;
    }
  }
  OK
}
