use http::{Extensions, HeaderMap};

use crate::Map;

impl Map for &HeaderMap {
  fn get(&self, key: impl AsRef<str>) -> Option<&str> {
    if let Some(v) = HeaderMap::get(self, key.as_ref())
      && let Ok(v) = v.to_str()
    {
      return Some(v);
    }
    None
  }
}

impl<T> crate::Req for volo_http::request::Request<T> {
  fn headers(&self) -> impl Map {
    self.headers()
  }

  fn extensions(&self) -> &Extensions {
    self.extensions()
  }

  fn extensions_mut(&mut self) -> &mut Extensions {
    self.extensions_mut()
  }
}
