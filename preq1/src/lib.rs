use bytes::Bytes;
use reqwest::{IntoUrl, RequestBuilder};

pub const MAX_RETRY: usize = 3;

pub async fn post(
  n: usize,
  client_li: &[reqwest::Client],
  url: impl IntoUrl,
  build: impl Fn(RequestBuilder) -> RequestBuilder,
) -> reqwest::Result<Bytes> {
  let mut retry = 0;
  let url = url.into_url()?;
  loop {
    let client = &client_li[(n.overflowing_add(retry)).0 % client_li.len()];

    macro_rules! ok {
      ($r:expr) => {{
        match $r.await {
          Ok(r) => Ok::<_, reqwest::Error>(r),
          Err(r) => {
            retry += 1;
            if retry >= MAX_RETRY {
              return Err(r);
            } else {
              tracing::warn!("{} RETRY {} : {}", url, retry, r);
              continue;
            }
          }
        }
      }};
    }

    if let Ok(r) = ok!(build(client.post(url.clone())).send())
      && let Ok(r) = ok!(r.bytes())
    {
      return Ok(r);
    }
  }
}

pub async fn post_form(
  n: usize,
  client_li: &[reqwest::Client],
  url: impl IntoUrl,
  form: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
) -> reqwest::Result<Bytes> {
  let url = url.into_url()?;
  let form = form
    .into_iter()
    .map(|(k, v)| {
      (k.as_ref().to_owned(), v.as_ref().to_owned())
      // format!(
      //   "{}={}",
      //   k.as_ref(),
      //   form_urlencoded::byte_serialize(v.as_ref().as_bytes()).collect::<String>()
      // )
    })
    .collect::<Vec<_>>();
  //   .join("&");
  post(n, client_li, url, |req| req.form(&form)).await
}
