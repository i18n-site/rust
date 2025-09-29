#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
pub(crate) mod net;
use aok::Result;
pub use error::Error;
use rand::Rng;

pub struct Aier {
  pub api: String,
  pub token_li: Vec<String>,
  pub token_pos: *mut usize,
}

impl Drop for Aier {
  fn drop(&mut self) {
    unsafe {
      let _ = Box::from_raw(self.token_pos);
    }
  }
}

impl Aier {
  pub fn new(api: impl Into<String>, token_li: Vec<String>) -> Self {
    let pos = Box::new(rand::rng().random_range(0..token_li.len()));
    Self {
      api: api.into(),
      token_li,
      token_pos: Box::into_raw(pos),
    }
  }
  pub async fn chat(&self) -> Result<String> {
    let mut pos = unsafe { *self.token_pos } + 1;
    if pos >= self.token_li.len() {
      pos = 0;
    }
    unsafe {
      *self.token_pos = pos;
    }

    let token = &self.token_li[pos];
    dbg!((pos, token));
    Ok("xx".into())
  }
}

//   fn increment(&self) {
//     unsafe {
//       *self.ptr += 1;
//     }
//   }
//
//   fn get_value(&self) -> usize {
//     unsafe { *self.ptr }
//   }
// }
