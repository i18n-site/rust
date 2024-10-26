use xstr::is_ascii_digit;

/// x.x.com -> x.com
pub fn host_tld(domain: impl AsRef<str>) -> String {
  let domain = domain.as_ref().as_bytes();
  let mut domain = domain;
  if let Some(d) = psl::domain(domain) {
    let bytes = d.suffix().as_bytes();
    let len = bytes.len();
    if len > 0 && !is_ascii_digit(bytes) {
      let mut n = domain.len() - len;
      n = n.saturating_sub(1);
      while n > 0 {
        let t = n - 1;
        if domain[t] == b'.' {
          break;
        }
        n = t;
      }
      domain = &domain[n..]
    }
  }
  unsafe { String::from_utf8_unchecked(domain.into()) }
}

/// x.x.com:1131 -> x.com , 这个函数还会去掉端口号
pub fn host_port_tld(host: impl AsRef<str>) -> String {
  let host = host.as_ref();
  host_tld(if let Some(p) = host.find(':') {
    &host[..p]
  } else {
    host
  })
}

pub fn url_host_port(url: impl AsRef<str>) -> String {
  let url = url.as_ref();
  let begin = if let Some(pos) = url.find("://") {
    pos + 3
  } else {
    0
  };
  let url = &url[begin..];
  let end = if let Some(pos) = url.find('/') {
    pos
  } else {
    url.len()
  };
  url[..end].into()
}

pub fn url_tld(url: impl AsRef<str>) -> String {
  host_port_tld(url_host_port(url))
}
