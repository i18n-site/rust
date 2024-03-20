use std::{
  collections::{HashMap, HashSet},
  path::Path,
};

use aok::{Result, OK};
use lang::Lang;
use redb::Database;
use walkdir::WalkDir;

use crate::{tran_path, FromTo};

pub async fn _tran_ext(
  token: &str,
  dir: &Path,
  from: Lang,
  from_to: &FromTo,
  ext: &str,
  traned_lang: &mut HashSet<Lang>,
  traned_file: &mut HashMap<String, HashSet<u16>>,
  db: &Database,
) -> Result<()> {
  if traned_lang.contains(&from) {
    return OK;
  }
  traned_lang.insert(from);
  if let Some(p) = from_to.tf.get(&from) {
    Box::pin(_tran_ext(
      token,
      dir,
      *p,
      from_to,
      ext,
      traned_lang,
      traned_file,
      db,
    ))
    .await?;
  }
  let root = dir.join(from_to.lang_str.get(&from).unwrap());
  let root_len = root.as_os_str().to_string_lossy().len() + 1;
  for entry in WalkDir::new(&root).into_iter().filter_entry(dot_hide::not) {
    if let Ok(entry) = xerr::ok!(entry) {
      let file_type = entry.file_type();
      if file_type.is_file() {
        let path = entry.path();
        if let Some(path) = path.to_str() {
          if path.ends_with(ext) {
            let key = &path[root_len..path.len() - ext.len()];
            let from_lang: u16 = from as _;

            let mut not_exist = true;
            let traned = traned_file
              .entry(key.into())
              .and_modify(|t| {
                not_exist = !t.contains(&from_lang);
                if not_exist {
                  t.insert(from_lang);
                }
              })
              .or_insert_with(|| HashSet::from([from_lang]));

            if not_exist {
              let rel = Path::new(path)
                .strip_prefix(&root)?
                .as_os_str()
                .to_string_lossy();
              for i in tran_path(token, from, &ext[1..], dir, &rel, from_to, db).await? {
                traned.insert(i);
              }
            }
          }
        }
      }
    }
  }

  /*
  扫描每个key, 如果key有上级, 就优先做上级
  */
  OK
}

pub async fn tran_ext(
  token: &str,
  dir: &Path,
  from_to: impl Into<FromTo>,
  ext: &str,
  db: &Database,
) -> Result<()> {
  /*
  扫描每个key, 如果key有上级, 就优先做上级
  */
  let from_to = from_to.into();
  let mut traned = HashSet::new();
  let mut traned_file = HashMap::new();
  let ext = ".".to_owned() + ext;

  for i in from_to.ft.keys().copied() {
    _tran_ext(
      token,
      dir,
      i,
      &from_to,
      &ext,
      &mut traned,
      &mut traned_file,
      db,
    )
    .await?;
  }

  OK
}
