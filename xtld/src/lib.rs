use xstr::is_ascii_digit;

/// x.x.com -> x.com
pub fn tld(domain: &str) -> &str {
  let bin = domain.as_bytes();
  let mut bin = bin;
  if let Some(d) = psl::domain(bin) {
    let bytes = d.suffix().as_bytes();
    let len = bytes.len();
    if len > 0 && !is_ascii_digit(bytes) {
      let mut n = bin.len() - len;
      n = n.saturating_sub(1);
      while n > 0 {
        let t = n - 1;
        if bin[t] == b'.' {
          break;
        }
        n = t;
      }
      bin = &bin[n..]
    }
  }

  &domain[domain.len() - bin.len()..]
  // unsafe { String::from_utf8_unchecked(domain.into()) }
}

/// x.x.com:1131 -> x.com , 这个函数还会去掉端口号
pub fn host_port_tld(host: &str) -> &str {
  tld(if let Some(p) = host.find(':') {
    &host[..p]
  } else {
    host
  })
}

pub fn url_host_port(url: &str) -> &str {
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

pub fn url_tld(url: &str) -> &str {
  host_port_tld(url_host_port(url))
}
