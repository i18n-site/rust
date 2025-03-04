use http::HeaderMap;

pub struct Cookie<'a> {
  pub domain: String,
  pub headers: &'a mut HeaderMap,
}

pub fn new<'a>(domain: impl Into<String>, headers: &'a mut HeaderMap) -> Cookie<'a> {
  Cookie {
    domain: domain.into(),
    headers,
  }
}

impl Cookie<'_> {
  pub fn set(&mut self, key: impl AsRef<str>, val: impl AsRef<str>, max_age: u64) {
    let key = key.as_ref();
    let val = val.as_ref();
    let cookie = format!(
      "{key}={val};Max-Age={max_age};Domain={};Secure;HttpOnly;Path=/;Partitioned",
      self.domain
    );
    if let Ok(cookie) = cookie.parse() {
      self.headers.append("set-cookie", cookie);
    }
  }
}
