#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]

#[derive(Default, Debug)]
pub struct TxtLi {
  pub li: Vec<String>,
  pub restore: Restore,
}

#[cfg(feature = "impl")]
pub mod restore;
#[cfg(feature = "impl")]
pub use restore::Restore;

#[cfg(feature = "impl")]
impl TxtLi {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn push_tran(&mut self, txt: impl Into<String>) {
    self.li.push(txt.into());
  }

  pub fn push_tran_line(&mut self, txt: impl Into<String>) {
    self.push_tran(txt);
    self.push_no_tran("\n");
  }

  pub fn push_no_tran(&mut self, txt: impl Into<String>) {
    self.restore.push(self.li.len(), txt.into());
  }

  pub fn push_no_tran_line(&mut self, txt: impl Into<String>) {
    self.push_no_tran(txt.into() + "\n");
  }

  #[cfg(feature = "push_md_line")]
  pub fn push_md_line(&mut self, txt: impl Into<String>) {
    let org = txt.into();
    let org_len = org.len();
    let mut txt = &org[..];
    let mut split_pos = org_len;

    let mut iter = txt.char_indices();
    let mut offset = 0;

    'out: while let Some((pos, i)) = iter.next() {
      macro_rules! jump {
        ($n: expr) => {{
          let n = $n;
          txt = &txt[n..];
          iter = txt.char_indices();
          offset += n;
          continue 'out;
        }};
      }

      if "-+".contains(i)
        && let Some(c) = txt[pos + 1..].chars().next()
      {
        if c.is_whitespace() || ".-|:".contains(c) {
          let _ = iter.next();
          continue;
        }
      } else if i == '_' {
        if txt[pos + 1..].chars().all(|c| c == '_') {
          split_pos = org_len;
          break;
        }
      } else if i == '*'
        && let Some(c) = txt[pos + 1..].chars().next()
      {
        if c != '*' {
          let _ = iter.next();
          continue;
        } else if txt[pos + 2..].chars().all(|c| c == '*') {
          split_pos = org_len;
          break;
        }
      } else if i == '[' {
        let p = pos + 1;
        let remain = &txt[p..];
        if remain.starts_with("x]") || remain.starts_with(" ]") {
          jump!(p + 2);
        } else if let Some(remain) = remain.strip_prefix("^") {
          for (pos2, c) in remain.char_indices() {
            if c == ']' {
              jump!(p + pos2 + 2);
            } else if c.is_whitespace() {
              continue 'out;
            }
          }
        }
        split_pos = pos + offset;
        break;
      }
      if !("#>.:|=".contains(i) || i.is_whitespace()) {
        split_pos = pos + offset;
        break;
      }
    }

    if split_pos > 0 {
      self.push_no_tran(&org[..split_pos]);
    }
    if split_pos < org_len {
      let remain = &org[split_pos..];
      if remain.len() == 1
        && let Some(c) = remain.chars().next()
        && !c.is_ascii_alphabetic()
      {
        self.push_no_tran(remain);
      } else {
        self.push_tran(remain);
      }
    }
    self.push_no_tran("\n");
  }
}
