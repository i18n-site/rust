use crate::{Kind, Md};

#[derive(Debug, Clone, Default)]
pub struct MdLi {
  pub li: Vec<Md>,
  pub txt_pos_li: Vec<usize>,
}

impl MdLi {
  pub fn txt_iter(&self) -> impl Iterator<Item = (usize, &str)> {
    self.txt_pos_li.iter().map(|pos| {
      let pos = *pos;
      (pos, self.li[pos].str.as_str())
    })
  }

  pub fn join(&self) -> String {
    self.li.iter().map(|md| md.str.as_str()).collect()
  }

  pub fn end_indent(&mut self) {
    let li = &mut self.li;
    if let Some(last) = li.last_mut() {
      if last.kind == Kind::Txt {
        let last_str = last.str.to_owned();
        let last_str_len = last_str.len();
        let last_trim_end = last_str.trim_end();
        let diff = last_str_len - last_trim_end.len();
        if diff > 0 {
          last.str = last_trim_end.to_string();
          li.push(Md {
            kind: Kind::EndIndent,
            str: (&last_str[last_str_len - diff..]).into(),
          })
        }
      }
    }
  }

  pub fn push_txt(&mut self, kind: Kind, str: impl Into<String>) {
    self.txt_pos_li.push(self.li.len());
    self.push(kind, str);
  }

  pub fn push(&mut self, kind: Kind, str: impl Into<String>) {
    if kind == Kind::Br {
      self.end_indent();
    }

    self.li.push(Md {
      kind,
      str: str.into(),
    });
  }
}
