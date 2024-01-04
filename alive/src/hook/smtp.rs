use std::{
  net::{IpAddr, SocketAddr},
  time::Duration,
};

use aok::{Result, OK};
use mail_send::{smtp::tls::build_tls_connector, SmtpClientBuilder};

use crate::{
  db::{Kind, Watch},
  dberr,
};

pub const SMTP_PORT: u16 = 587;

pub async fn ping<'a>(
  kind: &'a Kind,
  watch: &'a Watch,
  host: &'a str,
  _: &'a str, // kind_args: : &'a str,
  _: &'a str, // watch_arg: : &'a str,
  ip: IpAddr,
) -> Result<()> {
  let ip = ip.to_string();
  let smtp = SmtpClientBuilder {
    addr: format!("{}:{}", &ip, SMTP_PORT),
    timeout: Duration::from_secs(60),
    tls_connector: build_tls_connector(false),
    tls_hostname: host,
    tls_implicit: false,
    is_lmtp: false,
    local_host: ip.clone(),
    credentials: None,
    say_ehlo: true,
  };

  let r = smtp.connect().await?.ehlo(host).await;

  dbg!((ip, r));
  OK
}
