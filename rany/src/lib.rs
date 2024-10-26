#![feature(doc_cfg)]

use num_traits::cast::AsPrimitive;

pub trait Rany {
  fn alphabet(&self) -> &[u8];
  fn pos(&self, c: u8) -> Option<u64>;

  fn d(&self, s: impl AsRef<[u8]>) -> u64 {
    let s = s.as_ref();
    let alphabet = self.alphabet();
    let len = alphabet.len();
    let mut result = 0u64;
    for (i, c) in s.iter().rev().enumerate() {
      if let Some(digit) = self.pos(*c) {
        result += digit * (len.pow(i as _) as u64);
      } else {
        tracing::warn!("invalid char {:?}", c);
      }
    }

    result
  }

  fn e(&self, num: impl AsPrimitive<u64>) -> Vec<u8> {
    let mut num = num.as_();
    let alphabet = self.alphabet();
    if num == 0 {
      vec![alphabet[0]]
    } else {
      let len = alphabet.len() as u64;
      let mut result = Vec::new();

      while num != 0 {
        let remainder = num % len;
        result.push(alphabet[remainder as usize]);
        num /= len;
      }

      result.into_iter().rev().collect()
    }
  }
}

#[cfg(feature = "b255")]
mod b255;

#[cfg(feature = "b255")]
#[doc(cfg(feature = "b255"))]
pub use b255::{B255, B255_BIN};

#[cfg(feature = "strany")]
mod strany;

#[cfg(feature = "strany")]
#[doc(cfg(feature = "strany"))]
pub use strany::StrAny;

#[cfg(feature = "url")]
mod url;

#[cfg(feature = "url")]
#[doc(cfg(feature = "url"))]
pub use url::{URL, URL_BIN};
