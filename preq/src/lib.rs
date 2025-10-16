use bytes::Bytes;
use preq1::post_form;
use reqwest::IntoUrl;

static mut N: usize = 0;

pub struct Proxy;

impl Proxy {
  pub async fn post_form(
    url: impl IntoUrl,
    form: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
  ) -> reqwest::Result<Bytes> {
    unsafe { (N, _) = N.overflowing_add(1) };
    post_form(unsafe { N }, &reqwest_client::PROXY, url, form).await
  }
}
