pub struct Response {
  pub code: u16,
  pub body: bytes::Bytes,
}

impl std::error::Error for Response {}

impl std::fmt::Display for Response {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "err code {}", self.code)
  }
}

impl std::fmt::Debug for Response {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self, f)
  }
}
