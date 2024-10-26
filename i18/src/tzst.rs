use std::path::Path;

use aok::Result;
use prost::Message;

use crate::{api, RelFt};

pub fn tzst(
  workdir: &Path,
  path_li: &[String],
  lrs_li: Vec<api::LangRelSrcHash>,
  rel_ft: &[RelFt],
  url: &[String],
) -> Result<Vec<u8>> {
  let mut from_to_li = Vec::with_capacity(rel_ft.len() + 1);
  for (prefix, ft) in rel_ft {
    let mut from_to: std::collections::HashMap<_, _> = ft
      .ft
      .iter()
      .map(|(f, t)| {
        (
          (*f as u32),
          api::LangLi {
            li: t.iter().map(|i| *i as u32).collect(),
          },
        )
      })
      .collect();

    if let Some(default_from) = ft.default_from {
      from_to.insert(default_from as u32, api::LangLi { li: vec![] });
    }

    from_to_li.push(api::FromTo {
      prefix: prefix.into(),
      from_to,
    });
  }

  let meta = api::Meta {
    lrs_li,
    from_to_li,
    url: url.into(),
  };
  let mut w = tzst::W::new();
  w.add_bin("_", meta.encode_to_vec())?;
  w.add_path_li(workdir, path_li)?;
  Ok(w.end()?)
}
