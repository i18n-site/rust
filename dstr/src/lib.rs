#![cfg_attr(docsrs, feature(doc_cfg))]

use std::fmt::{Debug, Write};

pub fn dstr<T: Debug>(v: T) -> String {
  let mut buffer = String::new();
  write!(&mut buffer, "{v:?}").unwrap();
  buffer
}

#[cfg(feature = "dvec")]
#[macro_export]
macro_rules! dvec {
  ($($x:expr),* $(,)?) => {{
    use $crate::dstr;
    vec![$(dstr(&$x)),*]
  }};
}
