use cmp_len_str::cmp_len_str;

use crate::trie::ExtTrie;

fn strip_prefix(prefix: impl AsRef<str>, url: &str) -> Option<&str> {
  let prefix = prefix.as_ref();
  if prefix.is_empty() {
    return Some(url);
  }
  if let Some(c) = url.strip_prefix(prefix) {
    if let Some(c) = c.strip_prefix('/') {
      return Some(c);
    } else if c.is_empty() {
      return Some("");
    } else if let Some(p) = c.rfind('.') {
      if p == 0 {
        return Some(c);
      }
    }
  }
  None
}

#[derive(Debug, Clone)]
pub struct PrefixLi(pub Vec<(String, ExtTrie)>);

impl PrefixLi {
  pub fn new(mut prefix_li: Vec<String>) -> Self {
    prefix_li.sort_by(cmp_len_str);

    Self(
      prefix_li
        .into_iter()
        .map(|p| (p, Default::default()))
        .collect(),
    )
  }

  pub fn add(&mut self, rel: impl Into<String>, ver: impl Into<String>) {
    let rel = rel.into();

    for i in &mut self.0 {
      if let Some(rel) = strip_prefix(&i.0, &rel) {
        i.1.add(rel, ver);
        break;
      }
    }
  }
}
