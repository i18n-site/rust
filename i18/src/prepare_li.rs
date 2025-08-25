use i18_hash::File;

use crate::{api, Scan};

pub fn prepare_li(
  to_tran: Vec<File>,
  scan: &Scan,
) -> (Vec<api::LangRelSrcHash>, Vec<String>, Vec<File>) {
  let mut lrs_li = vec![];
  let to_tran_len = to_tran.len();
  let mut path_li = Vec::with_capacity(to_tran_len);
  let mut update_cache_file_li = Vec::with_capacity(to_tran_len);

  for i in to_tran {
    let rel = &i.rel;
    let lang = i.lang;
    let has_pre_hash = !i.pre_hash.is_empty();
    let tzst_path = !i.meta.to_li.is_empty() || has_pre_hash;
    if tzst_path {
      let path = format!("{}/{}", lang::LANG_CODE[i.lang as usize], rel);
      path_li.push(path);
    }
    // 如果没有源语言, 不需要更新翻译缓存
    if has_pre_hash {
      for (prefix, ft) in &scan.rel_ft {
        if rel.starts_with(prefix) {
          if ft.from(lang).is_some() {
            lrs_li.push(api::LangRelSrcHash {
              lang: lang as _,
              rel: rel.clone(),
              pre_hash: i.pre_hash.to_vec(),
            })
          }
          break;
        }
      }
    }
    if tzst_path {
      update_cache_file_li.push(i);
    }
  }
  (lrs_li, path_li, update_cache_file_li)
}
