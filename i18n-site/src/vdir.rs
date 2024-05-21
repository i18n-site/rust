use std::path::Path;

use aok::Result;
use daachorse::{DoubleArrayAhoCorasick, DoubleArrayAhoCorasickBuilder, MatchKind};
use walkdir::WalkDir;

use crate::{api, ExtUrlLiBuild};

pub struct VDir {
  pub ac: DoubleArrayAhoCorasick<usize>,
  pub prefix_ver_li: Vec<(
    String,      // prefix
    Vec<String>, //ver_li
    ExtUrlLiBuild,
  )>,
}

impl VDir {
  pub fn build(self, dir: &Path) -> Result<Vec<(api::Vtab, api::Vlang)>> {
    use prost::Message;
    let public = dir.join("public");
    let public_v = public.join("v");
    let len = self.prefix_ver_li.len();
    let mut vli = Vec::with_capacity(len);
    for (prefix, ver_li, ext_url_li) in self.prefix_ver_li {
      let vlang = api::Vlang {
        ext_url_li: ext_url_li.build(),
        lang_url_li: vec![],
      };

      let v_dir = if prefix.is_empty() {
        public_v.clone()
      } else {
        public_v.join(&prefix)
      };

      ifs::wbin(v_dir.join(&ver_li[0]), vlang.encode_to_vec())?;

      vli.push((
        api::Vtab {
          prefix,
          ver_li,
          vlang_url_li: vec![],
        },
        vlang,
      ));
    }
    vli.sort_by_key(|i| {
      let prefix = &i.0.prefix;
      std::cmp::Reverse((prefix.len(), prefix.clone()))
    });

    Ok(vli)
  }

  pub fn add(&mut self, rel: &str) {
    let pos = self.find(rel);
    let li = &mut self.prefix_ver_li[pos];

    let rfp = if li.0.is_empty() {
      rel
    } else {
      &rel[1 + li.0.len()..]
    };

    li.2.add(rfp);
  }

  pub fn find(&self, path: &str) -> usize {
    let mut it = self.ac.leftmost_find_iter(path);

    if let Some(m) = it.next() {
      if m.start() == 0 {
        let end = m.end();

        if end != path.len() && !path[end..].starts_with('/') {
          return 0;
        }

        return m.value();
      }
    }

    0
  }

  pub fn new(dir_v: &Path) -> Self {
    let mut li = vec![];
    let mut prefix_ver_li = vec![];

    if dir_v.exists() {
      for entry in WalkDir::new(dir_v).into_iter().filter_entry(dot_hide::not) {
        if let Ok(entry) = xerr::ok!(entry) {
          if entry.file_type().is_file() {
            if let Some(file_name) = entry.file_name().to_str() {
              if file_name == "v.yml" {
                let fp = entry.path();
                let dir = fp.parent().unwrap().strip_prefix(dir_v).unwrap();
                if let Some(dir) = dir.to_str() {
                  if let Ok(bin) = xerr::ok!(ifs::r(fp)) {
                    if let Ok(ver_li) = xerr::ok!(serde_yaml::from_slice::<Vec<String>>(&bin)) {
                      let dir = ifs::unix_path(dir);
                      let v = (dir.clone(), ver_li, ExtUrlLiBuild::default());
                      if dir.is_empty() {
                        prefix_ver_li.insert(0, v);
                      } else {
                        prefix_ver_li.push(v);
                        li.push((dir, li.len() + 1));
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }

    li.sort_by_key(|i| std::cmp::Reverse(i.0.len()));

    Self {
      prefix_ver_li,
      ac: DoubleArrayAhoCorasickBuilder::new()
        .match_kind(MatchKind::LeftmostLongest)
        .build_with_values(li)
        .unwrap(),
    }
  }
}
