use std::path::Path;

use aok::{Null, OK};

use crate::api;

pub trait Upload {
  fn add(&mut self, lang: lang::Lang, rel: &str);
  fn upload(
    self,
    site: api::Site,
    root: &Path,
    vlang_li: Vec<api::Vlang>,
    lang_bin_li: Vec<Vec<u8>>,
  ) -> impl std::future::Future<Output = Null> + Send;
}

#[derive(Default)]
pub struct No;

impl Upload for No {
  fn add(&mut self, _lang: lang::Lang, _rel: &str) {}
  async fn upload(
    self,
    _site: api::Site,
    _root: &Path,
    _vlang_li: Vec<api::Vlang>,
    _lang_bin_li: Vec<Vec<u8>>,
  ) -> Null {
    OK
  }
}
