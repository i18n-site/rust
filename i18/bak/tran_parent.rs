use std::{
  collections::{HashMap, HashSet},
  path::PathBuf,
};

use aok::{Result, OK};
use lang::Lang;

use crate::{tran_dir, LangStr, Watch};
// use crate::{api::PathLangMeta, tran};

pub async fn tran_parent(
  traned_from: &mut HashSet<Lang>,
  from_to: &HashMap<LangStr, Vec<LangStr>>,
  to_from: &HashMap<LangStr, LangStr>,
  dir: &PathBuf,
  from_lang: &LangStr,
  to_lang_li: &Vec<LangStr>,
  watch: &mut Watch,
) -> Result<()> {
  if traned_from.contains(&from_lang.lang) {
    return OK;
  }
  traned_from.insert(from_lang.lang);
  let no_parent = if let Some(p) = to_from.get(from_lang) {
    if let Some(to_lang_li) = from_to.get(p) {
      Box::pin(tran_parent(
        traned_from,
        from_to,
        to_from,
        dir,
        p,
        to_lang_li,
        watch,
      ))
      .await?;
    }
    false
  } else {
    true
  };
  tran_dir(from_lang, to_lang_li, dir, watch, no_parent).await?;
  OK
}
