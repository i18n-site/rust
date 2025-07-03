#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]

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
  pub fn new(api: String, token_li: Vec<String>) -> Self {
    let pos = Box::new(rand::rng().random_range(0..token_li.len()));
    Self {
      api,
      token_li,
      token_pos: unsafe { Box::into_raw(pos) },
    }
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
