use std::{fs, path::Path};

use bjs::{li_hashmap_to_jsvalue, li_str_to_jsvalue};
use lang::{Lang, LANG_CODE};
use tracing::error;

use crate::{api, Conf};

pub const FILE: &str = "file";
pub const NAV: &str = "nav";
pub const ARG: &str = "arg";

pub fn js(
  root: &Path,
  all_lang: &[Lang],
  conf: &Conf,
  nav_li: &mut Vec<api::Nav>,
  mut add: impl FnMut(Lang, String),
) {
  let js = root.join(".i18n/hook/after");
  if !js.exists() {
    return;
  }
  let mut file_li = vec![];
  for i in fs::read_dir(&js).unwrap() {
    if let Ok(i) = xerr::ok!(i) {
      if let Ok(file_type) = xerr::ok!(i.file_type()) {
        if file_type.is_file() {
          let i = i.path();
          if i.extension() == Some("js".as_ref()) {
            file_li.push(i);
          }
        }
      }
    }
  }
  if file_li.is_empty() {
    return;
  }

  let all_lang = all_lang
    .iter()
    .map(|i| LANG_CODE[*i as usize])
    .collect::<Vec<_>>();

  let ctx = &mut bjs::ctx(js.to_str().unwrap(), root);
  let arg = [
    li_hashmap_to_jsvalue(ctx, &conf.nav[..]),
    li_str_to_jsvalue(ctx, &all_lang),
  ];

  for file in file_li {
    println!("{}", file.display());
    match bjs::default(ctx, &file, &arg) {
      Ok(r) => {
        if let Ok(Some(nav)) = bjs::obj_get(&r, NAV) {
          if let Ok(mut nav) = bjs::obj2map(nav) {
            for i in &mut *nav_li {
              if let Some(obj) = nav.remove(&i.i18n) {
                if let Ok(Some(arg)) = bjs::obj_get(&obj, ARG) {
                  if let Some(arg) = bjs::to_str(arg) {
                    i.arg = Some(arg);
                  }
                }
              }
            }
          }
        }
        if let Ok(Some(file)) = bjs::obj_get(&r, FILE) {
          for (fp, txt) in bjs::li_str(ctx, file) {
            if let Some(lang_str) = fp.split("/").next() {
              if let Ok::<Lang, _>(lang) = xerr::ok!(lang_str.try_into()) {
                xerr::log!(ifs::wtxt(root.join(&fp), &txt));
                add(lang, fp[lang_str.len() + 1..].into());
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
}
