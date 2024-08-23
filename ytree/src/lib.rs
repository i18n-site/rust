#[cfg(feature = "lang")]
pub mod lang;

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
  pub fn iter(&self) -> LiIter {
    LiIter::new(self, String::new())
  }

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
          ._push(&rest_path);
      } else {
        let mut new_sub = IndexMap::new();
        let mut new_li = Li(Vec::new());
        new_li._push(&rest_path);
        new_sub.insert(key.clone(), new_li);
        self.0.insert(0, Node::Sub(new_sub));
      }
    }
  }
}

pub struct LiIter<'a> {
  parent: String,
  nodes: std::slice::Iter<'a, Node>,
  sub: Option<Box<LiIter<'a>>>,
  sub_map_iter: Option<indexmap::map::Iter<'a, String, Li>>,
}

impl<'a> LiIter<'a> {
  fn new(li: &'a Li, parent: String) -> Self {
    Self {
      parent,
      nodes: li.0.iter(),
      sub: None,
      sub_map_iter: None,
    }
  }
}

impl<'a> Iterator for LiIter<'a> {
  type Item = String;

  fn next(&mut self) -> Option<Self::Item> {
    // Continue with the current sub-iterator if it exists
    if let Some(iter) = &mut self.sub {
      if let Some(path) = iter.next() {
        return Some(path);
      } else {
        self.sub = None; // Finished current sub-iterator
      }
    }

    // Check if there is an ongoing sub_map_iter
    if let Some(iter) = &mut self.sub_map_iter {
      if let Some((path, li)) = iter.next() {
        let new_parent = format!("{}{}/", self.parent, path);
        self.sub = Some(Box::new(LiIter::new(li, new_parent)));
        return self.next();
      } else {
        self.sub_map_iter = None; // Finished iterating the current IndexMap
      }
    }

    // Process the next node in the current level
    while let Some(node) = self.nodes.next() {
      match node {
        Node::Sub(sub) => {
          self.sub_map_iter = Some(sub.iter());
          return self.next();
        }
        Node::File(file) => {
          return Some(format!("{}{}", self.parent, file));
        }
      }
    }
    None
  }
}
