use bytes::Bytes;
use preq1::{post_form, proxy};
use reqwest::IntoUrl;

genv::s!(
  IPV6_PROXY_IP_LI, IPV6_PROXY_PORT:u16,IPV6_PROXY_USER, IPV6_PROXY_PASSWD
);

genv::s!(IPV6_PROXY_PROTOCOL: String | "http".to_string());

static mut N: usize = 0;

pub struct Proxy(Vec<reqwest::Client>);

#[static_init::dynamic(lazy)]
pub static PROXY: Proxy = {
  let mut v = Vec::new();

  let protocol = &*IPV6_PROXY_PROTOCOL;

  let url = format!("{protocol}://{}:{}@", *IPV6_PROXY_USER, *IPV6_PROXY_PASSWD,);
  let port: u16 = *IPV6_PROXY_PORT;
  for ip in IPV6_PROXY_IP_LI.split_whitespace() {
    let url = format!("{url}{ip}:{port}",);
    v.push(proxy(
      if protocol == "https" {
        reqwest::Proxy::https(url)
      } else {
        reqwest::Proxy::http(url)
      }
      .unwrap(),
    ));
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
