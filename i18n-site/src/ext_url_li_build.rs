use std::collections::{HashMap, HashSet};

use crate::api;

#[derive(Debug, Default)]
pub struct ExtUrlLiBuild {
  pub url_set: HashSet<String>,
}

impl ExtUrlLiBuild {
  pub fn add(&mut self, path: impl Into<String>) {
    let path = path.into();
    self.url_set.insert(path);
  }

  pub fn build(&self) -> Vec<api::ExtUrlLi> {
    let mut ext_url_map = HashMap::new();
    for url in self.url_set.iter() {
      let name;
      let ext = if let Some(p) = url.rfind('.') {
        let ext = &url[p + 1..];
        let t = &url[..p];
        name = if ext == "md" && t == "README" { "" } else { t };
        ext
      } else {
        name = url;
        ""
      };

      ext_url_map.entry(ext).or_insert_with(Vec::new).push(name);
    }

    let mut li = ext_url_map
      .into_iter()
      .map(|(ext, li)| {
        let mut li = li.into_iter().map(|i| i.to_owned()).collect::<Vec<_>>();
        li.sort();
        (ext, li)
      })
      .collect::<Vec<_>>();
    li.sort();

    li.into_iter()
      .map(|(ext, url_li)| api::ExtUrlLi {
        ext: ext.into(),
        url_li,
      })
      .collect()
  }
}
