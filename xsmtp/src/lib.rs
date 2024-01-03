use mail_builder::{headers::address::Address, MessageBuilder};
use mail_send::SmtpClientBuilder;
use static_init::dynamic;

genv::def!(SMTP_PORT, SMTP_HOST, SMTP_USER, SMTP_PASSWORD);
genv::def!(SMTP_IMPLICIT_TLS:bool| false);
genv::s!(SMTP_FROM);

#[dynamic]
pub static SMTP: SmtpClientBuilder<String> = {
  let smtp_port: u16 = SMTP_PORT();
  let smtp_host = SMTP_HOST();
  let smtp_user = SMTP_USER();
  let smtp_password = SMTP_PASSWORD();

  SmtpClientBuilder::new(smtp_host, smtp_port)
    .implicit_tls(SMTP_IMPLICIT_TLS())
    .credentials((smtp_user, smtp_password))
};

pub async fn async_send(
  from_name: impl Into<String>,
  to: impl Into<Address<'static>>,
  subject: impl Into<String>,
  txt: impl Into<String>,
  htm: impl Into<String>,
) -> Result<(), mail_send::Error> {
  let subject = subject.into();
  let txt = txt.into();
  let htm = htm.into();
  let from_name = from_name.into();
  let to = to.into();

  let mut mail = MessageBuilder::new()
    .from((from_name.as_str(), SMTP_FROM.as_str()))
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

pub fn send(
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
  trt::bg(async_send(from_name, to, subject, txt, htm));
}
