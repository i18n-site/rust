#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]

use citer::CIter;

pub struct Aier {
  pub api: String,
  pub token_li: Vec<String>,
}

impl Aier {
  pub fn new(api: String, token_li: Vec<String>) -> Aier<'a> {
    Self {
      api,
      token_li,
      // token_iter: CIter::rand(&token_li),
    }
  }
}
