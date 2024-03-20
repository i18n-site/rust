use std::path::PathBuf;

use aok::{Result, OK};
use walkdir::WalkDir;

use crate::{tran, Change, LangStr, Watch};

pub async fn tran_dir(
  from_lang: &LangStr,
  to_lang_li: &Vec<LangStr>,
  dir: impl Into<PathBuf>,
  watch: &mut Watch,
  no_parent: bool,
) -> Result<()> {
  let dir = dir.into();
  let from_lang_str = &from_lang.str;
  let from_dir = dir.join(from_lang_str);
  let walker = WalkDir::new(&from_dir).into_iter();
  for entry in walker.filter_entry(dot_hide::not) {
    if let Ok(entry) = xerr::ok!(entry) {
      let file_type = entry.file_type();
      if file_type.is_file() {
        let path = entry.path();
        if let Some(ext) = path.extension() {
          if let Some(ext) = ext.to_str() {
            if ["yml", "md"].contains(&ext) {
              let rel = path.strip_prefix(&from_dir)?.as_os_str().to_string_lossy();
              let rel = rel.as_ref();

              for i in to_lang_li {
                let rel = format!("{}/{rel}", i.str);
                let to = dir.join(&rel);

                if let Change::True(txt) = &watch.is_change(&rel).await {
                  let hash = if let Some(src_hash) = &txt.src {
                    dbg!(("todo update tran", to, &txt, &src_hash));
                    src_hash.clone()
                  } else {
                    Box::from([])
                  };
                  watch.save(rel, &txt.v, hash)?;
                }
              }

              let from_rel = format!("{from_lang_str}/{rel}");
              if let Change::True(txt) = watch.is_change(&from_rel).await {
                dbg!((&from_rel, "change"));
                let hash = if no_parent {
                  watch.save(from_rel, &txt.v, [])?
                } else {
                  ifs::b3_len(txt.v.as_bytes())
                };

                for (i, traned) in to_lang_li.iter().zip(
                  tran(
                    from_lang.lang,
                    to_lang_li.iter().map(|i| i.lang).collect(),
                    &txt.v,
                  )
                  .await
                  .into_iter(),
                ) {
                  let rel = format!("{}/{rel}", i.str);
                  watch.save(rel, traned, &hash)?;
                }
              }
            }
          }
        }
      }
    }
  }
  OK
}
