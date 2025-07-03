#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]

use citer::CIter;

pub struct Aier<'a> {
  pub api: String,
  pub token_iter: CIter<'a, String>,
  pub token_li: Vec<String>,
}

impl<'a> Aier<'a> {
  pub fn new(api: String, token_li: Vec<String>) -> Aier<'a> {
    Self {
      api,
      token_li,
      token_iter: CIter::rand(&token_li),
    }
  }
}
