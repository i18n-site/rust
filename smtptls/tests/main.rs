use std::net::IpAddr;

use aok::{OK, Void};
use smtptls::smtptls;
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

genv::s!(
  SMTP_PASSWORD,
  SMTP_PORT,
  SMTP_FROM,
  SMTP_USER,
  SMTP_HOST,
  SMTP_IP
);

#[tokio::test]
async fn test() -> Void {
  let smtp_ip: IpAddr = SMTP_IP.parse()?;
  let port: u16 = SMTP_PORT.parse()?;

  info!("{}", &*SMTP_HOST);

  let remain_days = smtptls(
    SMTP_FROM.split('@').next_back().unwrap(),
    (smtp_ip, port),
    &SMTP_USER,
    &SMTP_PASSWORD,
    30,
  )
  .await?
    / 86400;

  info!(?remain_days);
  OK
}
