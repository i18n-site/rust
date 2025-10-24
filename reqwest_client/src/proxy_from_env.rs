use pos_next::PosNext;

use crate::{TIMEOUT, proxy};
genv::s!(
  IPV6_PROXY_IP_LI, IPV6_PROXY_PORT:u16,IPV6_PROXY_USER, IPV6_PROXY_PASSWD
);

genv::s!(IPV6_PROXY_PROTOCOL: String | "http".to_string());

#[static_init::dynamic]
pub static PROXY: Vec<reqwest::Client> = {
  let mut v = Vec::new();

  let protocol = &*IPV6_PROXY_PROTOCOL;

  let url = format!("{protocol}://{}:{}@", *IPV6_PROXY_USER, *IPV6_PROXY_PASSWD,);
  let port: u16 = *IPV6_PROXY_PORT;

  let timeout = std::time::Duration::from_secs(*TIMEOUT);
  for ip in IPV6_PROXY_IP_LI.split_whitespace() {
    let url = format!("{url}{ip}:{port}",);
    v.push(
      proxy(reqwest::Proxy::https(url).unwrap())
        .timeout(timeout)
        .build()
        .unwrap(),
    );
  }
  v
};

pub struct ProxyIter {
  pos: usize,
}

impl Default for ProxyIter {
  fn default() -> Self {
    Self::new()
  }
}

static mut N: usize = 0;

impl ProxyIter {
  pub fn new() -> ProxyIter {
    unsafe { (N, _) = N.overflowing_add(1) };
    ProxyIter { pos: unsafe { N } }
  }
}

impl Iterator for ProxyIter {
  type Item = &'static reqwest::Client;
  fn next(&mut self) -> Option<Self::Item> {
    let pos = self.pos;
    self.pos = pos.overflowing_add(1).0;
    Some(&PROXY[pos % PROXY.len()])
  }
}

pub fn proxy_iter() -> impl Fn() -> &'static reqwest::Client {
  unsafe { (N, _) = N.overflowing_add(1) };
  let pos = PosNext::new(unsafe { N });
  let len = PROXY.len();
  move || &PROXY[pos.next() % len]
}
