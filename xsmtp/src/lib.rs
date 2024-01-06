use std::net::{IpAddr, SocketAddr};

use mail_builder::{headers::address::Address, MessageBuilder};
use mail_send::SmtpClientBuilder;
use static_init::dynamic;

// SMTP HOST 请给域名而不是 IP

genv::def!(SMTP_HOST, SMTP_USER, SMTP_PASSWORD);
genv::def!(SMTP_IMPLICIT_TLS:bool| false);
genv::def!(SMTP_PORT:u16| 587);
genv::s!(SMTP_FROM);

pub fn smtp_builder(smtp_host: impl Into<String>, ip: IpAddr) -> SmtpClientBuilder<String> {
  let smtp_host = smtp_host.into();
  let smtp_user = SMTP_USER();
  let smtp_password = SMTP_PASSWORD();
  let smtp_port: u16 = SMTP_PORT();

  SmtpClientBuilder::new_bind_ip(smtp_host, ip, smtp_port)
    .implicit_tls(SMTP_IMPLICIT_TLS())
    .credentials((smtp_user, smtp_password))
}

pub static mut SMTP: Option<SmtpClientBuilder<String>> = None;

pub async fn send(
  from_name: impl AsRef<str>,
  to: impl Into<Address<'static>>,
  subject: impl AsRef<str>,
  txt: impl AsRef<str>,
  htm: impl AsRef<str>,
) -> Result<(), mail_send::Error> {
  no_retry_send(from_name, to, subject, txt, htm).await
}

pub async fn no_retry_send<T>(
  smtp: SmtpClientBuilder<T>
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
  let mut smtp = SMTP.connect().await?;
  smtp.send(mail).await?;
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
