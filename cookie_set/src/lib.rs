use http::HeaderValue;

const MAX: u32 = 99999999;

pub struct Cookie {
  pub domain: String,
}

pub fn new(domain: impl Into<String>) -> Cookie {
  Cookie {
    domain: domain.into(),
  }
}

impl Cookie {
  pub fn set(&self, key: impl AsRef<str>, val: impl AsRef<str>, max_age: u32) -> HeaderValue {
    let key = key.as_ref();
    let val = val.as_ref();
    let cookie = format!(
      "{key}={val};Max-Age={max_age};Domain={};Secure;HttpOnly;Path=/;Partitioned",
      self.domain
    );
    cookie.parse().unwrap()
  }

  pub fn set_max(&self, key: impl AsRef<str>, val: impl AsRef<str>) -> HeaderValue {
    self.set(key, val, MAX)
  }
}
