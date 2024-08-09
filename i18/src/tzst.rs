use std::path::Path;

use aok::Result;
use prost::Message;

use crate::{api, RelFt};

pub fn tzst(
  workdir: &Path,
  path_li: Vec<String>,
  lrs_li: Vec<api::LangRelSrcHash>,
  rel_ft: &[RelFt],
) -> Result<Vec<u8>> {
  let mut from_to_li = Vec::with_capacity(rel_ft.len() + 1);
  for (prefix, ft) in rel_ft {
    let from_to: std::collections::HashMap<_, _> = ft
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

    from_to_li.push(api::FromTo {
      prefix: prefix.into(),
      from_to,
    });
  }

  let meta = api::Meta { lrs_li, from_to_li };
  let mut w = tzst::W::new();
  w.add_bin("_", meta.encode_to_vec())?;
  w.add_path_li(workdir, path_li)?;
  Ok(w.end()?)
}
