#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]

use citer::CIter;

pub struct Aier<'a> {
  pub api: String,
  pub token_iter: CIter<'a, String>,
  pub token_li: Vec<String>,
}
