use std::borrow::Borrow;

pub use roaring::treemap::RoaringTreemap;

pub struct TranFmt {
  pub li: Vec<String>,
  pub line_pos: RoaringTreemap,
}

pub fn tran_fmt(txt: impl AsRef<str>) -> (RoaringTreemap, Vec<String>) {
  let mut line_pos = RoaringTreemap::new();
  let mut li = vec![];
  for (pos, i) in txt.as_ref().lines().enumerate() {
    let i = i.trim_end();
    if i.is_empty() {
      continue;
    }
    li.push(i.into());
    line_pos.insert(pos as _);
  }
  (line_pos, li)
}

pub fn restore<S: AsRef<str>>(
  line_pos: impl Borrow<RoaringTreemap>,
  li: impl IntoIterator<Item = S>,
) -> String {
  let mut r = String::new();
  let mut pre = 0;
  for (pos, i) in line_pos.borrow().iter().zip(li.into_iter()) {
    let pos = pos as usize;
    while pre < pos {
      r.push('\n');
      pre += 1;
    }
    r.push_str(i.as_ref());
  }
  r
}
