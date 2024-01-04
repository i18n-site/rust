use std::net::SocketAddr;

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
  addr: SocketAddr,
) -> Result<()> {
  dbg!(host, watch.dns_type);

  OK
}
