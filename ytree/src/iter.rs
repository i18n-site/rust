use crate::{Li, Node};

impl Li {
  pub fn iter(&self) -> LiIter {
    LiIter::new(self, String::new())
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
    if let Some(node) = self.nodes.next() {
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
