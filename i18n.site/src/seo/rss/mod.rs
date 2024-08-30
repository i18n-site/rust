use std::path::PathBuf;

use gxhash::{HashMap, HashMapExt};
use lang::{Lang, LANG_CODE};
use ytree::sitemap::{LangRelTs, LangSet, Sitemap};

mod topk;
pub use topk::topk;
mod xml;
pub use xml::Xml;

pub struct Rss {
  pub host: String,
  pub root: PathBuf,
  pub lang_rel_ts: LangRelTs,
  pub li: HashMap<Lang, Vec<(String, String, String)>>,
}

impl Rss {
  pub fn new(root: impl Into<PathBuf>, host: impl Into<String>, lang_rel_ts: LangRelTs) -> Self {
    Self {
      lang_rel_ts,
      host: host.into(),
      root: root.into(),
      li: Default::default(),
    }
  }

  pub fn gen(&self) -> impl IntoIterator<Item = (String, String)> + use<'_> {
    let mut lang_rel_ts = convert_rel_lang_set(&self.lang_rel_ts);
    self.li.iter().filter_map(move |(lang, rel_title_htm)| {
      let lang = *lang;
      if let Some(mut rel_ts) = lang_rel_ts.remove(&(lang as u32)) {
        let lang_en = LANG_CODE[lang as usize];
        let mut xml = Xml::new(&self.host, self.root.join(lang_en), lang_en);
        let mut limit = 3;
        for (rel, title, htm) in rel_title_htm {
          if limit == 0 {
            break;
          }
          if let Some(ts) = rel_ts.remove(rel) {
            limit -= 1;
            xml.add(ts, rel, title, htm);
          }
        }
        if limit > 0 {
          {
            dbg!(&rel_ts);
            let min = rel_ts.values().min().unwrap();

            for (rel, ts) in &rel_ts {
              dbg!((rel, ts - min));
            }
          }

          for (rel, ts) in topk(limit, rel_ts) {
            xml.add_rel(ts, &rel);
          }
        }
        Some((format!("{lang_en}.rss"), xml.gen()))
      } else {
        None
      }
    })
  }

  pub fn push(
    &mut self,
    lang: Lang,
    rel: impl Into<String>,
    title: impl Into<String>,
    htm: impl Into<String>,
  ) {
    let rel = rel.into();

    if !self.lang_rel_ts.contains(lang, &rel) {
      let title = title.into();
      let htm = htm.into();
      self.li.entry(lang).or_default().push((rel, title, htm));
    }
  }
}
