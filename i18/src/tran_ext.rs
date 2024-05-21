use std::{collections::HashSet, path::Path};

use aok::Result;
use ft::FromTo;
use globset::GlobSet;
use lang::{Lang, LANG_CODE};
use redb::Database;
use walkdir::WalkDir;

use crate::{need_tran, need_tran::NeedTran};

pub fn _tran_ext(
  ignore: &GlobSet,
  dir: &Path,
  from: Lang,
  from_to: &FromTo,
  ext: &str,
  db: &Database,
  traned: &mut HashSet<String>,
) -> Result<Vec<NeedTran>> {
  let root = dir.join(LANG_CODE[from as usize]);
  let root_len = root.as_os_str().to_string_lossy().len() + 1;
  let pure_ext = &ext[1..];
  let mut li = vec![];
  if !root.exists() {
    return Ok(li);
  }
  for entry in WalkDir::new(&root).into_iter().filter_entry(dot_hide::not) {
    if let Ok(entry) = xerr::ok!(entry) {
      let file_type = entry.file_type();
      if file_type.is_file() {
        let path = entry.path();

        if let Some(path) = path.to_str() {
          if path.ends_with(ext) {
            let key = &path[root_len..path.len() - ext.len()];
            if traned.contains(key) {
              continue;
            }

            let rel = Path::new(path)
              .strip_prefix(&root)?
              .as_os_str()
              .to_string_lossy()
              .replace('\\', "/");

            if ignore.is_match(String::from("/") + &rel) {
              continue;
            }

            traned.insert(key.into());
            let need_tran = need_tran(db, pure_ext, dir, from_to, rel)?;
            if need_tran.len > 0 {
              li.push(need_tran);
            }
          }
        }
      }
    }
  }

  Ok(li)
}

pub fn tran_ext(
  ignore: &GlobSet,
  dir: &Path,
  from_to: impl Into<FromTo>,
  ext: &str,
  db: &Database,
) -> Result<Vec<NeedTran>> {
  /*
  扫描每个key, 如果key有上级, 就优先做上级
  */
  let from_to = from_to.into();
  let dot_ext = ".".to_owned() + ext;

  let mut li = vec![];
  let mut traned_lang = HashSet::new();
  let mut traned = HashSet::new();

  macro_rules! _tran_ext {
    ($lang:ident) => {{
      let lang = $lang;
      traned_lang.insert(lang);
      li.extend(_tran_ext(
        ignore,
        dir,
        lang,
        &from_to,
        &dot_ext,
        &db,
        &mut traned,
      )?);
    }};
  }

  for i in from_to.from_lang_li() {
    while let Some(i) = from_to.from(i)
      && !traned_lang.contains(&i)
    {
      _tran_ext!(i);
    }
    _tran_ext!(i);
  }

  Ok(li)
}
