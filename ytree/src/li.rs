use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Node {
  Sub(IndexMap<String, Li>),
  File(String),
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Li(pub Vec<Node>);

impl Li {
  pub fn from_iter<S: AsRef<str>>(iter: impl IntoIterator<Item = S>) -> Self {
    let mut me: Self = Default::default();
    for i in iter {
      me.push(i);
    }
    me
  }

  pub fn contains(&self, path: impl AsRef<str>) -> bool {
    let parts: Vec<&str> = path.as_ref().split('/').collect();
    self.contains_recursive(&parts)
  }

  fn contains_recursive(&self, parts: &[&str]) -> bool {
    let len = parts.len();
    if len >= 1 {
      let mut iter = self.0.iter();
      if let Some(n) = iter.next() {
        match n {
          Node::File(file) => {
            if len == 1 {
              return file == parts[0];
            }
          }
          Node::Sub(sub) => {
            if len > 1 {
              if let Some(sub_li) = sub.get(parts[0]) {
                return sub_li.contains_recursive(&parts[1..]);
              }
            }
          }
        }
      }
      if len == 1 {
        return iter.any(|node| match node {
          Node::File(file) => file == parts[0],
          _ => false,
        });
      }
    }

    false
  }

  pub fn push(&mut self, path: impl AsRef<str>) {
    let path = path.as_ref().split("/").collect::<Vec<_>>();
    self._push(&path);
  }

  fn _push(&mut self, path: &[&str]) {
    let len = path.len();
    if len == 1 {
      self.0.push(Node::File(path[0].to_string()));
    } else if len > 1 {
      let key = path[0].to_owned();
      let rest_path = &path[1..];

      if let Some(Node::Sub(sub)) = self.0.first_mut() {
        sub
          .entry(key.clone())
          .or_insert_with(|| Li(Vec::new()))
          ._push(rest_path);
      } else {
        let mut new_sub = IndexMap::new();
        let mut new_li = Li(Vec::new());
        new_li._push(rest_path);
        new_sub.insert(key.clone(), new_li);
        self.0.insert(0, Node::Sub(new_sub));
      }
    }
  }
}

impl Li {
  pub fn remove(&mut self, path: impl AsRef<str>) -> bool {
    let parts: Vec<&str> = path.as_ref().split('/').collect();
    self.remove_recursive(&parts)
  }

  fn remove_recursive(&mut self, parts: &[&str]) -> bool {
    let len = parts.len();
    if len == 0 {
      return false;
    }

    let part = parts[0];
    if len == 1 {
      if let Some(pos) = self.0.iter().position(|node| match node {
        Node::File(file) => file == part,
        _ => false,
      }) {
        self.0.remove(pos);
        return true;
      }
    } else {
      // 查找第一个Node::Sub并解构出其中的IndexMap
      if let Some(Node::Sub(sub)) = self.0.iter_mut().find(|node| match node {
        Node::Sub(_) => true,
        _ => false,
      }) {
        if let Some(sub_li) = sub.get_mut(part) {
          if sub_li.remove_recursive(&parts[1..]) {
            if sub_li.0.is_empty() {
              sub.shift_remove(part);
            }
            return true;
          }
        }
      }
    }

    false
  }
}
