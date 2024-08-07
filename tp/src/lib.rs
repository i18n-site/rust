#![feature(doc_cfg)]

use roaring::RoaringTreemap;

#[derive(Debug, Clone, Default)]
pub struct TxtPos<'a> {
  pub txt_li: Vec<&'a str>,
  pub pos_li: RoaringTreemap,
}

#[cfg(feature = "mut")]
#[doc(cfg(feature = "mut"))]
pub mod pos;

#[cfg(feature = "mut")]
#[doc(cfg(feature = "mut"))]
pub mod trim;

#[cfg(feature = "mut")]
#[doc(cfg(feature = "mut"))]
mod txtpos;

#[cfg(feature = "mut")]
#[doc(cfg(feature = "mut"))]
pub fn whitespace_or_quote(c: char) -> bool {
  c.is_whitespace() || "\"'".contains(c)
}

#[cfg(feature = "extend")]
#[doc(cfg(feature = "extend"))]
mod extend;
