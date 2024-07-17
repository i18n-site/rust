use std::{collections::HashMap, ops::AddAssign};

#[derive(Debug, Default)]
pub struct VerCount {
  pub ver_count: HashMap<String, usize>,
  pub ver_li: Vec<String>,
}

#[derive(Debug)]
pub struct VerMap {
  pub li: Vec<String>,
  pub pos: HashMap<usize, usize>,
}

impl VerCount {
  pub fn push(&mut self, ver: impl Into<String>) {
    let ver = ver.into();
    self.ver_li.push(ver.clone());
    self.ver_count.entry(ver).or_insert(0).add_assign(1);
  }

  pub fn map(self) -> VerMap {
    let mut vc = self.ver_count.into_iter().collect::<Vec<_>>();
    vc.sort_by(|a, b| {
      let cmp = b.1.cmp(&a.1);
      if cmp == std::cmp::Ordering::Equal {
        b.0.cmp(&a.0)
      } else {
        cmp
      }
    });
    let li = vc.into_iter().map(|i| i.0).collect::<Vec<String>>();
    let mut pos = HashMap::default();
    for (vpos, ver) in self.ver_li.into_iter().enumerate() {
      let p = li.iter().position(|i| **i == *ver).unwrap();
      if p != 0 {
        pos.insert(vpos, p);
      }
    }
    VerMap { li, pos }
  }
}
