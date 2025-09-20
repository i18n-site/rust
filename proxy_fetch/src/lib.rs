#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]

use base64::{Engine as _, engine::general_purpose::STANDARD};
use futures::future;
use reqwest::Url;

mod error;
pub use error::{Error, Result};
mod proxy;
pub use proxy::Proxy;

#[derive(Debug)]
pub struct ProxyFetch {
  pub subscription_url_li: Vec<Url>,
}

impl ProxyFetch {
  pub async fn load<U>(subscription_url_li: impl IntoIterator<Item = U>) -> Result<Self>
  where
    U: TryInto<Url, Error = url::ParseError>,
  {
    let subscription_url_li = subscription_url_li
      .into_iter()
      .map(|v| v.try_into())
      .collect::<std::result::Result<Vec<_>, _>>()?;

    let client = reqwest::Client::builder()
      .danger_accept_invalid_certs(true)
      .build()?;
    let bodies = future::join_all(subscription_url_li.iter().map(|url| {
      let client = client.clone();
      async move {
        if let Ok(resp) = xerr::ok!(client.get(url.clone()).send().await) {
          resp.text().await.unwrap_or_default()
        } else {
          "".into()
        }
      }
    }))
    .await;

    for body in bodies {
      let decoded = STANDARD.decode(body)?;
      let decoded = String::from_utf8_lossy(&decoded);
      for line in decoded.lines() {
        if line.starts_with("ss://") {
          println!("{line}");
        }
      }
    }

    Ok(Self {
      subscription_url_li,
    })
  }
}
