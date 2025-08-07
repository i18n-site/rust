use std::path::PathBuf;

use gxhash::HashMap;
use lang::{Lang, LANG_CODE};
use ytree::sitemap::LangRelTs;

mod topk;
pub use topk::topk;
mod xml;
pub use xml::Xml;

pub struct Rss {
  pub host: String,
  pub root: PathBuf,
  pub lang_rel_ts: LangRelTs,
  pub li: HashMap<Lang, Vec<(String, String, String)>>,
  pub now: u64,
}

pub const LIMIT: usize = 10;

impl Rss {
  pub fn new(root: impl Into<PathBuf>, host: impl Into<String>, lang_rel_ts: LangRelTs) -> Self {
    let now = sts::sec();
    Self {
      now,
      lang_rel_ts,
      host: host.into(),
      root: root.into(),
      li: Default::default(),
    }
  }

  pub fn dumps(mut self) -> String {
    let now = self.now;
    for (lang, li) in self.li {
      let entry = self.lang_rel_ts.lang_rel.entry(lang).or_default();
      for (rel, ..) in li {
        entry.insert(rel.clone());
        self.lang_rel_ts.rel_ts.insert(rel, now);
      }
    }
    self.lang_rel_ts.dumps()
  }

  pub fn gen(&self) -> impl IntoIterator<Item = (String, String)> + use<'_> {
    self.li.iter().filter_map(move |(lang, rel_title_htm)| {
      let lang = *lang;
      let lang_en = LANG_CODE[lang as usize];
      let mut xml = Xml::new(&self.host, self.root.join(lang_en), lang_en);
      let mut limit = LIMIT;
      for (rel, title, htm) in rel_title_htm {
        if limit == 0 {
          break;
        }
        limit -= 1;
        xml.add(self.now, rel, title, htm);
      }

      if limit != LIMIT {
        if limit > 0 {
          for (rel, ts) in topk(limit, &self.lang_rel_ts.rel_ts) {
            xml.add_rel(ts, rel);
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
      let entry = self.li.entry(lang).or_default();
      if entry.len() <= LIMIT {
        let title = title.into();
        let htm = htm.into();
        entry.push((rel, title, htm));
      }
    }
  }
}
