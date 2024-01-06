use std::{net::IpAddr, time::Duration};

use aok::{Result, OK};
use mail_send::{smtp::tls::build_tls_connector, SmtpClientBuilder};

use crate::db::{Kind, Watch};

pub const SMTP_PORT: u16 = 587;

pub async fn ping<'a>(
  _kind: &'a Kind,
  _watch: &'a Watch,
  host: &'a str,
  _: &'a str, // kind_args: : &'a str,
  _: &'a str, // watch_arg: : &'a str,
  ip: IpAddr,
) -> Result<()> {
  let host = host.to_owned();
  let smtp = SmtpClientBuilder::new_bind_ip(host, ip, SMTP_PORT);
  let ehlo = smtp.connect().await?.ehlo(host).await?;
  assert_eq!(ehlo.hostname, host);
  OK
}
