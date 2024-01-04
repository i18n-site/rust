use std::net::IpAddr;

use aok::{Result, OK};

use crate::{
  db::{Kind, Watch},
  dberr,
};

pub async fn ping<'a>(
  kind: &'a Kind,
  watch: &'a Watch,
  host: &'a str,
  _: &'a str, // kind_args: : &'a str,
  _: &'a str, // watch_arg: : &'a str,
  ip: IpAddr,
) -> Result<()> {
  OK
}
