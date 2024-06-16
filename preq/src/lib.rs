use bytes::Bytes;
use preq1::{post_form, proxy, IPV6_PROXY_PORT};
use reqwest::IntoUrl;

genv::s!(IPV6_PROXY);

static mut N: usize = 0;

pub struct Proxy(Vec<reqwest::Client>);

#[static_init::dynamic(lazy)]
pub static PROXY: Proxy = {
  let mut v = Vec::new();
  let port: u16 = IPV6_PROXY_PORT();
  for i in IPV6_PROXY.split(' ') {
    v.push(proxy(format!("{i}:{port}")));
  }
  Proxy(v)
};

impl Proxy {
  pub async fn post_form(
    &self,
    url: impl IntoUrl,
    form: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
  ) -> reqwest::Result<Bytes> {
    unsafe { (N, _) = N.overflowing_add(1) };
    post_form(unsafe { N }, &self.0, url, form).await
  }
}
