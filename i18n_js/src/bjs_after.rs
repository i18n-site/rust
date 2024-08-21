use std::{
  collections::HashMap,
  path::{Path, PathBuf},
};

use aok::Result;
use bjs::{
  boa_engine::{object::builtins::JsArray, string::JsString},
  JsMap,
};
use indexmap::IndexMap;
use lang::{Lang, LANG_CODE};
use tracing::error;

use crate::OUT;

pub const FILE: &str = "file";

pub type Lpb = HashMap<Lang, IndexMap<String, Box<[u8]>>>;

#[derive(Debug, Default)]
pub struct BjsAfter {
  pub lang_path_bin: Lpb,
}

pub fn bjs_after(
  root: &Path,
  lang_li: &[Lang],
  conf_name: &str,
  js_dir: &Path,
  after_tran: &[PathBuf],
) -> Result<BjsAfter> {
  if after_tran.is_empty() {
    return Ok(Default::default());
  }

  // lang_li
  // .iter()
  // .map(|i| lang::LANG_CODE[*i as usize])
  // .collect::<Vec<_>>();

  let ctx = &mut bjs::ctx(js_dir.to_str().unwrap(), root);
  let lang_li = {
    let li = JsArray::new(ctx);
    for lang in lang_li {
      li.push(JsString::from(LANG_CODE[*lang as usize]), ctx)
        .unwrap();
    }
    li
  };

  let mut map = JsMap::new(ctx);
  for (k, v) in [
    (
      "out",
      root.join(OUT).join(conf_name).to_str().unwrap_or_default(),
    ),
    ("root", root.to_str().unwrap_or_default()),
  ] {
    map.set_str(k, v);
  }
  map.set("lang_li", lang_li);

  let arg = &[map.value()];
  let mut lang_path_bin = Lpb::new();

  for file in after_tran {
    println!("{}", file.display());
    match bjs::default(ctx, js_dir.join(file), arg) {
      Ok(r) => {
        if let Ok(Some(file)) = bjs::obj_get(&r, FILE) {
          let mut li = bjs::li_str(ctx, file);
          li.sort_by(|a, b| a.0.cmp(&b.0));

          for (fp, txt) in li {
            if let Some(lang_str) = fp.split("/").next() {
              if let Ok::<Lang, _>(lang) = xerr::ok!(lang_str.try_into()) {
                lang_path_bin
                  .entry(lang)
                  .or_default()
                  .insert(fp[lang_str.len() + 1..].into(), txt.as_bytes().into());
              } else {
                error!("{fp} ( lang not found )");
              }
            }
          }
        }
      }
      Err(err) => {
        error!("{err}");
      }
    }
  }
  Ok(BjsAfter { lang_path_bin })
}
