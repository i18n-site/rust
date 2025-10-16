use std::{
  fmt,
  hash::{Hash, Hasher},
  net::SocketAddr,
};

#[derive(Debug, Clone)]
pub enum Proxy {
  Sock5(SocketAddr),
  Http(SocketAddr),
}

impl Proxy {
  pub fn addr(&self) -> &SocketAddr {
    match self {
      Proxy::Sock5(addr) => addr,
      Proxy::Http(addr) => addr,
    }
  }
}

impl Hash for Proxy {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.addr().hash(state);
  }
}

impl PartialEq for Proxy {
  fn eq(&self, other: &Self) -> bool {
    self.addr() == other.addr()
  }
}

impl Eq for Proxy {}

impl Ord for Proxy {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.addr().cmp(other.addr())
  }
}

impl PartialOrd for Proxy {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl fmt::Display for Proxy {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Proxy::Sock5(addr) => write!(f, "socks5://{addr}"),
      Proxy::Http(addr) => write!(f, "http://{addr}"),
    }
  }
}
