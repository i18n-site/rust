use crate::{Kind, Md};

#[derive(Debug, Clone)]
pub struct MdLi {
  pub li: Vec<Md>,
  pub pre_is_break: bool,
}

impl Default for MdLi {
  fn default() -> Self {
    Self {
      li: vec![],
      pre_is_break: true,
    }
  }
}

// pub pre_line: 0;
// pub pre_line: 0;

pub const TABLE_SPLIT: &str = "|";

impl MdLi {
  // pub fn txt_iter(&self) -> impl Iterator<Item = (usize, &str)> {
  //   self.txt_pos_li.iter().map(|pos| {
  //     let pos = *pos;
  //     (pos, self.li[pos].str.as_str())
  //   })
  // }

  pub fn join(&self) -> String {
    self.li.iter().map(|md| md.str.as_str()).collect()
  }

  pub fn push_txt(&mut self, kind: Kind, str: impl Into<String>) {
    let str = str.into();
    let trim = str.trim_start();
    let trim_len = trim.len();
    let diff = str.len() - trim_len;
    if diff > 0 {
      self.push(Kind::Space, &str[..diff]);
    }
    if !trim.is_empty() {
      for i in ["|-|", "|+|"] {
        if trim.starts_with(i) {
          self.push(Kind::Symbol, &trim[..3]);
          self.push_txt(kind, &trim[3..]);
          return;
        }
      }
      // 表格
      if trim.starts_with(TABLE_SPLIT) {
        self.push_break(Kind::TableSplit, TABLE_SPLIT);
        if trim.len() > 1 {
          for i in trim[1..].split(TABLE_SPLIT) {
            self.push_txt(kind, i);
            self.push_break(Kind::TableSplit, TABLE_SPLIT);
          }
          self.li.pop();
        }
        return;
      }

      if self.pre_is_break {
        // 1. 2. 10. 这种
        for (p, c) in trim.char_indices() {
          if c.is_ascii_digit() {
            continue;
          }
          if c == '.' {
            let p = p + 1;
            self.push(Kind::Symbol, &trim[..p]);
            self.push_txt(kind, &trim[p..]);
            return;
          } else {
            break;
          }
        }

        // 避免切分到不完整的unicode字符引发异常
        let trim_bytes = trim.as_bytes();
        let split: &[u8] = &trim_bytes[..2];
        if matches!(split, b"**" | b"__" | b"~~") {
          let split = &trim[..2];
          let remain = &trim[2..];
          if let Some(pos) = remain.find(split) {
            let end = &remain[pos + 2..];
            // 当行只包含一个 "**" 或 "__" 时
            if end.trim().is_empty() {
              self.push(Kind::Symbol, split);
              self.push_txt(kind, &remain[..pos]);
              self.push(Kind::Symbol, split);
              if !end.is_empty() {
                self.push(Kind::Space, end);
              }
              return;
            }
          }
        }

        'o: loop {
          for (p, c) in trim.char_indices() {
            if !"\\~#-+.>|:-=*_".contains(c) {
              let pre = &trim[..p];
              if !pre.is_empty() {
                self.push(Kind::Symbol, pre);
                self.push_txt(kind, &trim[p..]);
                return;
              }
              break 'o;
            }
          }
          self.push(Kind::Symbol, trim);
          return;
        }

        self.pre_is_break = false;
      }

      let str = trim;

      let trim = str.trim_end();

      if !trim.is_empty() {
        self.push(kind, trim);
      }

      let trim_len = trim.len();
      if trim_len < str.len() {
        self.push(Kind::Space, &str[trim_len..]);
      }
    }
  }

  pub fn push_break(&mut self, kind: Kind, str: impl Into<String>) {
    self.push(kind, str);
    self.pre_is_break = true;
  }

  pub fn push(&mut self, kind: Kind, str: impl Into<String>) {
    self.li.push(Md {
      kind,
      str: str.into(),
    });
  }
}
