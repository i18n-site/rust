#![feature(const_trait_impl)]
#![feature(effects)]

use std::net::IpAddr;

use aok::OK;
use mail_builder::{headers::address::Address, MessageBuilder};
use mail_send::SmtpClientBuilder;
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Error, Debug)]
pub enum XsmtpError {
  #[error("{0} DNS NO IP")]
  DnsNoIp(String),
}

// SMTP HOST 请给域名而不是 IP

genv::def!(SMTP_HOST, SMTP_USER, SMTP_PASSWORD);
genv::def!(SMTP_IMPLICIT_TLS:bool| false);
genv::def!(SMTP_PORT:u16| 587);
genv::s!(SMTP_FROM);

pub fn smtp_builder(host: impl Into<String>, ip: IpAddr) -> SmtpClientBuilder<String> {
  let smtp_user = SMTP_USER();
  let smtp_password = SMTP_PASSWORD();
  let smtp_port: u16 = SMTP_PORT();

  SmtpClientBuilder::new_bind_ip(host.into(), ip, smtp_port)
    .implicit_tls(SMTP_IMPLICIT_TLS())
    .credentials((smtp_user, smtp_password))
}

#[static_init::dynamic]
pub static SMTP: RwLock<Option<SmtpClientBuilder<String>>> = RwLock::new(None);

pub async fn send(
  from_name: impl AsRef<str>,
  to: impl Into<Address<'static>>,
  subject: impl AsRef<str>,
  txt: impl AsRef<str>,
  htm: impl AsRef<str>,
) -> aok::Result<()> {
  let from_name = from_name.as_ref();
  let to = to.into();
  let subject = subject.as_ref();
  let txt = txt.as_ref();
  let htm = htm.as_ref();

  macro_rules! send {
    ($smtp:ident) => {
      if let Err(err) = no_retry_send(&$smtp, from_name, to.clone(), subject, txt, htm).await {
        tracing::error!("SMTP {err}");
      } else {
        return OK;
      }
    };
  }

  {
    if let Some(smtp) = &*SMTP.read().await {
      send!(smtp);
    }
  }

  let host: String = SMTP_HOST();
  let ip_li = idns::ip(&host).await?;
  for i in ip_li {
    let smtp = smtp_builder(host.to_owned(), i);
    send!(smtp);
  }

  Err(XsmtpError::DnsNoIp(SMTP_HOST()))?

  // if let Some(smtp) = SMTP.as_ref() {
  //   return Ok(no_retry_send(smtp, from_name, to, subject, txt, htm).await?);
  // }
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
