pub const SET_COOKIE: &str = "Set-Cookie";

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
  pub fn set_max_for_js(&self, key: impl AsRef<str>, val: impl AsRef<str>) -> String {
    self.set_for_js(key, val, MAX)
  }

  pub fn set_for_js(&self, key: impl AsRef<str>, val: impl AsRef<str>, max_age: u32) -> String {
    let key = key.as_ref();
    let val = val.as_ref();
    format!(
      "{key}={val};Max-Age={max_age};Domain={};Secure;Path=/;Partitioned",
      self.domain
    )
  }
  pub fn set(&self, key: impl AsRef<str>, val: impl AsRef<str>, max_age: u32) -> String {
    let key = key.as_ref();
    let val = val.as_ref();
    format!(
      "{key}={val};Max-Age={max_age};Domain={};Secure;Path=/;Partitioned;HttpOnly",
      self.domain
    )
  }

  pub fn set_max(&self, key: impl AsRef<str>, val: impl AsRef<str>) -> String {
    self.set(key, val, MAX)
  }
}
