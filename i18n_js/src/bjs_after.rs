use std::{collections::HashMap, fs, path::Path};

use aok::Result;
use bjs::{li_hashmap_to_jsvalue, li_str_to_jsvalue};
use indexmap::IndexMap;
use lang::{Lang, LANG_CODE};
use tracing::error;

pub const FILE: &str = "file";
// pub const NAV: &str = "nav";
// pub const ARG: &str = "arg";

pub type Lpb = HashMap<Lang, IndexMap<String, Box<[u8]>>>;

#[derive(Debug, Default)]
pub struct BjsAfter {
  pub lang_path_bin: Lpb,
}

pub fn bjs_after(root: &Path, nav_li: &[crate::Nav], lang_li: &[Lang]) -> Result<BjsAfter> {
  let js = root.join(".i18n/hook/after.tran");
  if !js.exists() {
    return Ok(Default::default());
  }
  let mut file_li = vec![];
  for i in fs::read_dir(&js)? {
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
    return Ok(Default::default());
  }

  let lang_li = lang_li
    .iter()
    .map(|i| LANG_CODE[*i as usize])
    .collect::<Vec<_>>();

  let ctx = &mut bjs::ctx(js.to_str().unwrap(), root);
  let nav_li = nav_li
    .iter()
    .map(|i| {
      HashMap::from([
        ("i18n", i.i18n.clone()),
        ("url", i.url.clone()),
        ("use", i.r#use.clone()),
        ("menu", i.menu.clone()),
      ])
    })
    .collect::<Vec<_>>();

  let arg = [
    li_hashmap_to_jsvalue(ctx, &nav_li),
    li_str_to_jsvalue(ctx, &lang_li),
  ];

  let mut lang_path_bin = Lpb::new();

  for file in file_li {
    println!("{}", file.display());
    match bjs::default(ctx, &file, &arg) {
      Ok(r) => {
        // if let Ok(Some(nav)) = bjs::obj_get(&r, NAV) {
        //   if let Ok(mut nav) = bjs::obj2map(nav) {
        //     for i in &mut *nav_li {
        //       if let Some(obj) = nav.remove(&i.i18n) {
        //         if let Ok(Some(arg)) = bjs::obj_get(&obj, ARG) {
        //           if let Some(arg) = bjs::to_str(arg) {
        //             i.arg = Some(arg);
        //           }
        //         }
        //       }
        //     }
        //   }
        // }
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
                // lang_path_bin.insert(fp, txt.as_bytes().into());
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
