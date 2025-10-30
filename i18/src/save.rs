use std::path::PathBuf;

use ft::FromTo;
use gxhash::{HashMap, HashSet};
use i18_hash::{I18Hash, Meta};
use lang::LANG_CODE;
use roaring::bitmap::RoaringBitmap;
use xhash::xhash;

use crate::api;

pub struct Save {
  pub i18_hash: I18Hash,
  pub root: PathBuf,
  /// rel -> Waiting
  pub waiting: HashMap<String, Waiting>,
  pub rel_ft: Vec<(String, FromTo)>,
  pub pbar: pbar::Pbar,
  pub writed: Vec<String>,
}

#[derive(Default, Debug)]
pub struct Waiting {
  pub lang_meta: HashMap<u16, Meta>,
  pub to_li: RoaringBitmap,
  pub len: u64,
}

impl Save {
  pub fn new(
    root: impl Into<PathBuf>,
    mut i18_hash: I18Hash,
    rel_ft: Vec<(String, FromTo)>,
    to_update_path_hash: Vec<i18_hash::File>,
    updated_cache: &[String],
  ) -> Self {
    let updated_cache = HashSet::from_iter(updated_cache);
    let mut utime_path_li = vec![];
    let mut update_hash = HashMap::default();
    let mut waiting = HashMap::default();
    let mut total = 0;
    for file in to_update_path_hash {
      let lang = file.lang;
      let rel = &file.rel;
      let entry: &mut Waiting = waiting.entry(rel.clone()).or_default();
      for (prefix, ft) in &rel_ft {
        if rel.starts_with(prefix) {
          let to_li = ft.to_li_recursive(lang);
          // 如果只是更新缓存, Save初始化的时候已经更新了, 就设置
          if to_li.is_empty() {
            let fp = format!("{}/{}", LANG_CODE[lang as usize], rel);
            if updated_cache.contains(&fp) {
              update_hash
                .entry(rel.clone())
                .or_insert_with(Vec::new)
                .push((lang, file.meta));
            }
            utime_path_li.push(fp);
          } else {
            total += (to_li.len() as u64) * file.meta.len;
            if entry.len == 0 {
              entry.len = file.meta.len;
            }
            for i in to_li {
              entry.to_li.insert(i as _);
            }
            entry.lang_meta.insert(lang, file.meta);
          }
          break;
        }
      }
    }

    for (rel, li) in update_hash {
      xerr::log!(i18_hash.save(rel, li));
    }

    let pbar = if total > 0 {
      pbar::pbar(total as _)
    } else {
      pbar::pbar_no_run(0)
    };

    Save {
      rel_ft,
      root: root.into(),
      i18_hash,
      waiting,
      pbar,
      writed: vec![],
    }
  }

  pub fn has_waiting(&self) -> bool {
    !self.waiting.is_empty()
  }

  pub fn save(
    &mut self,
    traned: &std::collections::HashMap<String, api::TranedLi>,
  ) -> std::io::Result<bool> {
    let mut utime_path_li = vec![];
    let mut update_mtime_fp = vec![];
    let mut update_hash = HashMap::default();
    let ts = sts::sec();
    for (rel, api::TranedLi { li }) in traned {
      for api::Traned { lang, txt } in li {
        let path = format!("{}/{}", LANG_CODE[*lang as usize], rel);
        self.pbar.set_message(path.clone());
        ifs::wstr(self.root.join(&path), txt)?;
        self.writed.push(path.clone());
        update_mtime_fp.push(path.clone());
        if let Some(w) = self.waiting.get_mut(rel) {
          self.pbar.inc(w.len as _);
          let lang = { *lang };
          let update_hash = update_hash.entry(rel.clone()).or_insert_with(Vec::new);
          if !w.lang_meta.contains_key(&(lang as u16)) {
            for (prefix, ft) in &self.rel_ft {
              if rel.starts_with(prefix) {
                update_hash.push((
                  lang as u16,
                  Meta {
                    hash: xhash(txt),
                    ts,
                    len: txt.len() as u64,
                    to_li: if let Some(to_li) = ft.to_li(lang as u16) {
                      vb::e(to_li.into_iter().map(|i| i as u64).collect::<Vec<_>>())
                    } else {
                      Default::default()
                    },
                  },
                ));
                break;
              }
            }
          }
          w.to_li.remove(lang);
          if w.to_li.is_empty() {
            for (lang, meta) in w.lang_meta.drain() {
              let path = format!("{}/{}", LANG_CODE[lang as usize], rel);
              utime_path_li.push(path);
              update_hash.push((lang, meta));
            }
            self.waiting.remove(rel);
          }
        }
      }
    }

    if !update_mtime_fp.is_empty() {
      for (rel, li) in update_hash {
        xerr::log!(self.i18_hash.save(rel, li));
      }
    }
    Ok(self.has_waiting())
  }

  pub fn end(&mut self) {
    self.pbar.finish_and_clear();
    if self.has_waiting() {
      println!("❌ MISS TRANSLATE");
      crate::print_li::stderr(
        self.waiting.values().map(|w| w.to_li.len() as usize).sum(),
        self.waiting.iter().flat_map(|(rel, w)| {
          w.to_li
            .iter()
            .map(move |lang| format!("  {}/{}", LANG_CODE[lang as usize], rel))
        }),
      );
    }
  }
}
