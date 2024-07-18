use num_traits::cast::AsPrimitive;

use crate::Rany;

pub struct StrAny<'a>(pub &'a [u8]);

impl<'a> StrAny<'a> {
  pub fn dstr(&self, s: impl AsRef<str>) -> u64 {
    self.d(s.as_ref().as_bytes())
  }

  pub fn estr(&self, num: impl AsPrimitive<u64>) -> String {
    String::from_utf8(self.e(num)).unwrap()
  }
}

impl Rany for StrAny<'_> {
  fn alphabet(&self) -> &[u8] {
    self.0
  }

  fn pos(&self, c: u8) -> Option<u64> {
    self.0.iter().position(|x| *x == c).map(|i| i as _)
  }
}
