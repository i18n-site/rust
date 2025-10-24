#![cfg_attr(docsrs, feature(doc_cfg))]

mod post;
use std::sync::{
  Arc,
  atomic::{AtomicI16, Ordering},
};

use aok::{OK, Result};
use defer_lite::defer;
pub use post::{Answer, DohError, post};
use tokio::time::{Duration, sleep};
use tracing::warn;

pub mod record_type {
  pub const TXT: u16 = 16;
}

pub static DOH_LI: &[&str] = &[
  "doh.pub/resolve", // 腾讯
  "dns.google/resolve",
  "dns0.eu",
  "cloudflare-dns.com/dns-query",
  "doh.sb/dns-query",
  "doh.360.cn/resolve",
  "dns.nextdns.io",
  "dns.twnic.tw/dns-query",
  "dns.alidns.com/resolve",
];

pub async fn resolve<T: Send + 'static>(
  name: impl AsRef<str>,
  record_type: impl AsRef<str>,
  extract: impl Fn(Vec<Answer>) -> Result<Option<T>> + Send + 'static + Clone,
) -> Result<T> {
  let query = format!("?name={}&type={}", name.as_ref(), record_type.as_ref());
  let (send, recv) = kanal::bounded_async(1);

  let spawn = tokio::spawn(async move {
    let ing = Arc::new(AtomicI16::new(DOH_LI.len() as _));

    for doh in riter::iter(DOH_LI) {
      let query = query.clone();
      let send = send.clone();
      let ing = ing.clone();
      let extract = extract.clone();
      tokio::spawn(async move {
        let r = post(doh, &query).await;
        ing.fetch_sub(1, Ordering::Relaxed);

        macro_rules! send_err {
          ($err: expr) => {{
            let runing = ing.load(Ordering::Relaxed);
            if runing >= 0 {
              let err = $err;
              warn!("{runing} : {doh} {err}");
              if runing <= 1 {
                xerr::log!(send.send(Err(err)).await);
              }
            }
          }};
        }

        match r {
          Ok(res) => match extract(res) {
            Ok(res) => {
              if let Some(res) = res {
                let _ = send.send(Ok::<_, aok::Error>(res)).await;
                ing.store(-1, Ordering::Relaxed);
              }
            }
            Err(err) => send_err!(err),
          },
          Err(err) => {
            send_err!(err);
          }
        }
        OK
      });
      sleep(Duration::from_millis(500)).await;
    }
    OK
  });

  defer! {
    spawn.abort();
  }

  recv.recv().await?
}
