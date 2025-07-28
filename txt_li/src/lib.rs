#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]

#[derive(Default, Debug)]
pub struct TxtLi {
  pub li: Vec<String>,
  pub restore: Restore,
}

pub mod restore;
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
  pub fn push_md_trim_start_line(&mut self, txt: impl Into<String>) {
    use find_close_bucket::find_close_bucket;
    let txt = txt.into();
    for prefix in ["[", "!["] {
      if let Some(remain) = txt.strip_prefix(prefix) {
        if let Some(p) = find_close_bucket(remain, '[', ']')
          && p + 1 < remain.len()
          && let Some(bucket) = remain[p + 1..].chars().next()
          && let Some(bucket_close) = match bucket {
            '(' => Some(')'),
            '[' => Some(']'),
            _ => None,
          }
        {
          let url = &remain[p + 2..];
          if let Some(end) = find_close_bucket(url, bucket, bucket_close)
            && end + 1 == url.len()
          {
            let prefix_len = prefix.len();
            self.push_no_tran(prefix);
            let end = p + prefix_len;
            let t = txt[prefix_len..end].trim();
            if !t.is_empty() {
              self.push_md_trim_start_line(t);
            }
            self.push_no_tran(&txt[end..]);
            return;
          }
        }
        break;
      }
    }

    self.push_tran(txt);
  }

  #[cfg(feature = "push_md_line")]
  pub fn push_md_line(&mut self, txt: impl Into<String>) {
    let txt = txt.into();
    let mut 小括号 = false;
    let mut 中括号 = false;
    let mut 转义 = false;
    let mut offset = 0;

    for (pos, i) in txt.char_indices() {
      if 转义 {
        转义 = false;
      } else if i == '|' {
        if 小括号 || 中括号 {
          continue;
        }
        let t = &txt[offset..pos];
        let trim_end = t.trim_end();
        self.push_md(trim_end);

        let end = &t[trim_end.len()..];
        if !end.is_empty() {
          self.push_no_tran(end);
        }
        self.push_no_tran("|");
        offset = pos + 1;
      } else if i == '\\' {
        转义 = true;
      } else if i == '(' {
        小括号 = true;
      } else if i == '[' {
        中括号 = true;
      } else if i == ')' {
        小括号 = false;
      } else if i == ']' {
        中括号 = false;
      }
    }
    if offset < txt.len() {
      self.push_md(&txt[offset..]);
    }
    self.push_no_tran("\n");
  }

  #[cfg(feature = "push_md_line")]
  pub fn push_md(&mut self, txt: impl Into<String>) {
    let org = txt.into();

    let org_len = org.len();

    if org.starts_with("`") && org_len > 1 {
      let mut iter = org[1..].char_indices();
      while let Some((p, i)) = iter.next() {
        if i == '`' {
          let p = p + 2;
          if p == org_len {
            self.push_no_tran(org);
            return;
          }
          break;
        }
        if i == '\\' {
          iter.next();
        }
      }
    } else if let Some(remain) = org.strip_prefix("<") {
      if remain.starts_with("!--") && remain.ends_with("-->") {
        self.push_no_tran(org);
        return;
      }

      for tag in ["script", "code", "pre"] {
        if let Some(remain) = remain.strip_prefix(tag) {
          if let Some(next) = remain.chars().next()
            && (next.is_whitespace() || next == '>')
          {
            let remain = &remain[next.len_utf8()..];
            let mut find_close = find_close::FindClose::new(tag);
            if let Some(p) = find_close.find(remain)
              && p == remain.len()
            {
              self.push_no_tran(org);
              return;
            }
          }
          break;
        }
      }
    }

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

      if i.is_ascii_digit() {
        let p = pos + 1;
        for (pos2, c) in txt[p..].char_indices() {
          if c.is_ascii_digit() {
            continue;
          }
          if c == '.' {
            jump!(p + pos2 + 1);
          }
          break;
        }
      } else if "-+".contains(i)
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
        } else {
          let t = &txt[pos + 2..];
          if t.chars().all(|c| c == '*') {
            // ******
            split_pos = org_len;
            break;
          } else if let Some(p) = t.find("**")
            && p + 2 == t.len()
          {
            let p = pos + 2 + offset;
            self.push_no_tran(&org[..p]);
            self.push_md(&org[p..org_len - 2]);
            self.push_no_tran("**");
            return;
          }
        }
      } else if i == '[' {
        let p = pos + 1;
        let remain = &txt[p..];
        if remain.starts_with("x]") || remain.starts_with(" ]") {
          jump!(p + 2);
        } else {
          // [1] http://xxx
          for (pos2, c) in remain.char_indices() {
            if c.is_ascii_digit() {
              continue;
            }
            if c == ']' {
              jump!(p + pos2 + 1);
            }
            break;
          }
          let mut iter = remain.chars();
          while let Some(c) = iter.next() {
            if c.is_whitespace() {
              break;
            }
            if c == ']' {
              if let Some(next) = iter.next()
                && next == ':'
              {
                self.push_no_tran(org);
                return;
              }
              break;
            }
          }
        }
        split_pos = pos + offset;
        break;
      } else if unic_emoji_char::is_emoji(i) {
        continue;
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
        self.push_md_trim_start_line(remain);
      }
    }
  }
}
