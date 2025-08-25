#![feature(const_trait_impl)]
use std::net::IpAddr;

use aok::OK;
use mail_builder::{MessageBuilder, headers::address::Address};
use mail_send::SmtpClientBuilder;
use rand::{Rng, SeedableRng, rngs::StdRng};
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Error, Debug)]
pub enum XsmtpError {
  #[error("{0} DNS NO IP")]
  DnsNoIp(String),
}

// SMTP HOST 请给域名而不是 IP

genv::s!(SMTP_IMPLICIT_TLS:bool| false);
genv::s!(SMTP_PORT:u16| 587);
genv::s!(SMTP_HOST, SMTP_FROM, SMTP_USER, SMTP_PASSWORD);

pub fn smtp_builder(host: impl Into<String>, ip: IpAddr) -> SmtpClientBuilder<String> {
  let smtp_user = SMTP_USER.as_str();
  let smtp_password = SMTP_PASSWORD.as_str();
  let smtp_port: u16 = *SMTP_PORT;

  SmtpClientBuilder::new_bind_ip(host.into(), ip, smtp_port)
    .implicit_tls(*SMTP_IMPLICIT_TLS)
    .credentials((smtp_user.into(), smtp_password.into()))
}

#[static_init::dynamic]
pub static SMTP: RwLock<Option<SmtpClientBuilder<String>>> = RwLock::new(None);

pub async fn send_with_ipli(
  from_name: impl AsRef<str>,
  to: impl Into<Address<'static>>,
  subject: impl AsRef<str>,
  txt: impl AsRef<str>,
  htm: impl AsRef<str>,
  ip_li: Vec<IpAddr>,
) -> aok::Result<()> {
  let from_name = from_name.as_ref();
  let to = to.into();
  let subject = subject.as_ref();
  let txt = txt.as_ref();
  let htm = htm.as_ref();

  {
    if let Some(smtp) = &*SMTP.read().await {
      if let Err(err) = no_retry_send(smtp, from_name, to.clone(), subject, txt, htm).await {
        tracing::error!("SMTP {err}");
      } else {
        return OK;
      }
    }
  }

  let host = SMTP_HOST.as_str();
  let len = ip_li.len();
  if len == 0 {
    Err(XsmtpError::DnsNoIp(host.into()))?
  }

  let mut rng = StdRng::from_rng(&mut rand::rng());
  let mut pos = rng.random_range(0..len);

  let mut remain = len;
  loop {
    remain -= 1;
    let ip = ip_li[pos % ip_li.len()];
    pos += 1;
    let smtp = smtp_builder(host, ip);
    if let Err(err) = no_retry_send(&smtp, from_name, to.clone(), subject, txt, htm).await {
      if remain == 0 {
        *SMTP.write().await = None;
        return Err(err)?;
      }
      tracing::error!("{host} SMTP {err}");
    } else {
      *SMTP.write().await = Some(smtp);
      return OK;
    }
  }
}

pub async fn send(
  from_name: impl AsRef<str>,
  to: impl Into<Address<'static>>,
  subject: impl AsRef<str>,
  txt: impl AsRef<str>,
  htm: impl AsRef<str>,
) -> aok::Result<()> {
  let ip_li = idns::ip(&*SMTP_HOST).await?;
  send_with_ipli(from_name, to, subject, txt, htm, ip_li).await
}

pub async fn no_retry_send(
  smtp: &SmtpClientBuilder<String>,
  from_name: impl AsRef<str>,
  to: impl Into<Address<'static>>,
  subject: impl AsRef<str>,
  txt: impl AsRef<str>,
  htm: impl AsRef<str>,
) -> Result<(), mail_send::Error> {
  let from_name = from_name.as_ref();
  let subject = subject.as_ref();
  let txt = txt.as_ref();
  let htm = htm.as_ref();
  let to = to.into();

  let mut mail = MessageBuilder::new()
    .from((from_name, SMTP_FROM.as_str()))
    .to(to)
    .subject(subject);
  if !txt.is_empty() {
    mail = mail.text_body(txt);
  }
  if !htm.is_empty() {
    mail = mail.html_body(htm);
  }
  smtp.connect().await?.send(mail).await?;
  Ok(())
}

pub fn send_bg(
  from_name: impl Into<String>,
  to: impl Into<Address<'static>>,
  subject: impl Into<String>,
  txt: impl Into<String>,
  htm: impl Into<String>,
) {
  let subject = subject.into();
  let txt = txt.into();
  let htm = htm.into();
  let from_name = from_name.into();
  let to = to.into();
  trt::bg(send(from_name, to, subject, txt, htm));
}
