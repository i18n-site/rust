use std::collections::HashMap;

use serde::{Deserialize, Serialize, Serializer};
use serde_yaml;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Node {
  Sub(HashMap<String, Li>),
  File(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Li(pub Vec<Node>);

impl Li {
  pub fn contains(&self, path: impl AsRef<str>) -> bool {
    let parts: Vec<&str> = path.as_ref().split('/').collect();
    self.contains_recursive(&parts)
  }

  fn contains_recursive(&self, parts: &[&str]) -> bool {
    if parts.is_empty() {
      return false;
    }

    let mut iter = self.0.iter();

    if let Some(n) = iter.next() {
      match n {
        Node::File(file) => {
          if parts.len() == 1 {
            return file == parts[0];
          }
        }
        Node::Sub(sub) => {
          if let Some(sub_li) = sub.get(parts[0]) {
            return sub_li.contains_recursive(&parts[1..]);
          }
        }
      }
    }

    if parts.len() == 1 {
      return iter.any(|node| match node {
        Node::File(file) => file == parts[0],
        _ => false,
      });
    }

    false
  }

  pub fn insert(&mut self, path: Vec<&str>) {
    if path.len() == 1 {
      self.0.push(Node::File(path[0].to_string()));
    } else {
      let key = path[0].to_string();
      let rest_path = path[1..].to_vec();

      if let Some(Node::Sub(sub)) = self.0.first_mut() {
        sub
          .entry(key.clone())
          .or_insert_with(|| {
            let mut new_li = Li(Vec::new());
            new_li.insert(rest_path.clone());
            new_li
          })
          .insert(rest_path);
      } else {
        let mut new_sub = HashMap::new();
        let mut new_li = Li(Vec::new());
        new_li.insert(rest_path);
        new_sub.insert(key.clone(), new_li);
        self.0.insert(0, Node::Sub(new_sub));
      }
    }
  }
}
