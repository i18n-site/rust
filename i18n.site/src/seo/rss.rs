use gxhash::HashMap;
use lang::Lang;
use ytree::sitemap::Sitemap;

use super::Seo;

pub struct Rss {
  pub exist: Sitemap,
  pub rss: HashMap<Lang, HashMap<String, (String, String)>>,
}

impl Rss {
  pub fn new(exist: Sitemap) -> Self {
    Self {
      exist,
      rss: Default::default(),
    }
  }

  pub fn push(
    &mut self,
    lang: Lang,
    rel: impl Into<String>,
    title: impl Into<String>,
    htm: impl Into<String>,
  ) {
    let rel = rel.into();

    if !self.exist.contains(lang, &rel) {
      let title = title.into();
      let htm = htm.into();
      self.rss.entry(lang).or_default().insert(rel, (title, htm));
    }
  }
}
