#![cfg_attr(docsrs, feature(doc_cfg))]

#[derive(Default, Debug)]
pub struct TxtLi {
  pub li: Vec<String>,
  pub restore: Restore,
}

#[cfg(feature = "push_md_line")]
use htm_tag::htm_tag;

pub mod restore;
pub use restore::Restore;

#[cfg(feature = "push_md_line")]
pub const REMOVE: &str = "»:";

#[cfg(feature = "push_md_line")]
pub const NO_TRAN_TAG: [&str; 3] = ["script", "code", "pre"];

#[cfg(feature = "push_md_line")]
pub fn is_remove_char(char: char) -> Option<usize> {
  use unicode_properties::{GeneralCategory, UnicodeEmoji, UnicodeGeneralCategory};

  let len = char.len_utf8();

  if (len > 1 && char.is_emoji_char())
    || matches!(
      char.general_category(),
      GeneralCategory::Control
        | GeneralCategory::Format
        | GeneralCategory::NonspacingMark
        | GeneralCategory::LineSeparator
        | GeneralCategory::ParagraphSeparator
        | GeneralCategory::SpaceSeparator
    )
    || REMOVE.contains(char)
  {
    Some(len)
  } else {
    None
  }
}

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
              self.push_md(t);
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
  pub fn push_md(&mut self, txt: impl AsRef<str>) {
    let txt = txt.as_ref();

    let txt_len = txt.len();
    if txt_len == 0 {
      return;
    } else if let Some(remain) = txt.strip_prefix("`") {
      let mut iter = remain.char_indices();
      while let Some((p, i)) = iter.next() {
        if i == '`' {
          let p = p + 2;
          if p == txt_len {
            self.push_no_tran(txt);
            return;
          }
          break;
        }
        if i == '\\' {
          iter.next();
        }
      }
    } else if let Some(remain) = txt.strip_prefix("<") {
      if remain.starts_with("!--") && remain.ends_with("-->") {
        self.push_no_tran(txt);
        return;
      }

      let tag = htm_tag(remain);

      let no_tran_tag = if let Some(tag) = tag {
        NO_TRAN_TAG.contains(&tag)
      } else {
        false
      };

      if ((tag.is_some() && !no_tran_tag)
        || (remain.starts_with("/")
          && if let Some(tag) = htm_tag(&remain[1..]) {
            !NO_TRAN_TAG.contains(&tag)
          } else {
            false
          }))
        && let Some(p) = remain.find(">")
        && p + 1 == remain.len()
      {
        self.push_no_tran(txt);
        return;
      }

      if let Some(tag) = tag {
        if tag == "br"
          && let Some(p) = remain.find(">")
        {
          let p = p + 1;
          self.push_no_tran(&txt[..p]);
          self.push_md(&txt[p..]);
          return;
        } else if no_tran_tag {
          if let Some(remain) = remain.strip_prefix(tag)
            && let Some(next) = remain.chars().next()
            && (next.is_whitespace() || next == '>')
          {
            let remain = &remain[next.len_utf8()..];
            let mut find_close = find_close::FindClose::new(tag);
            if let Some(p) = find_close.find(remain)
              && p == remain.len()
            {
              self.push_no_tran(txt);
              return;
            }
          }
        } else {
          let mut find_close = find_close::FindClose::new(tag);
          if let Some(p) = find_close.find(remain)
            && p == remain.len()
          {
            let offset = tag.len() + 1;
            if let Some(begin) = txt[offset..].find(">") {
              let end = txt.len() - tag.len() - 1;
              if let Some(end) = txt[..end].rfind("<") {
                let begin = begin + offset + 1;
                if begin <= end {
                  self.push_no_tran(&txt[..begin]);
                  self.push_md(&txt[begin..end]);
                  self.push_no_tran(&txt[end..]);
                  return;
                }
              }
            }
          }
        }
      }
    }

    {
      // 移除开头的表情符号
      let mut remove = 0;
      for i in txt.chars() {
        if ".#=>:|·".contains(i) {
          remove += i.len_utf8();
        } else if let Some(len) = is_remove_char(i) {
          remove += len;
        } else {
          break;
        }
      }
      if remove > 0 {
        self.push_no_tran(&txt[..remove]);
        self.push_md(&txt[remove..]);
        return;
      }
    }

    {
      // 移除结尾的表情符号
      let mut remove = 0;
      for i in txt.chars().rev() {
        if ":-".contains(i) {
          remove += 1;
        } else if let Some(len) = is_remove_char(i) {
          remove += len;
        } else {
          break;
        }
      }
      if remove > 0 {
        let end = txt_len - remove;
        self.push_md(&txt[..end]);
        self.push_no_tran(&txt[end..]);
        return;
      }
    }

    if let Some((pos, i)) = txt.char_indices().next() {
      macro_rules! split {
        ($n: expr) => {{
          let n = $n;
          self.push_no_tran(&txt[..n]);
          self.push_md(&txt[n..]);
          return;
        }};
      }
      macro_rules! tran_middle {
        ($begin: expr, $end:expr) => {{
          let begin = $begin;
          let end = $end;
          self.push_no_tran(&txt[..begin]);
          self.push_md(&txt[begin..end]);
          self.push_no_tran(&txt[end..]);
          return;
        }};
      }
      if i.is_ascii_digit() {
        let p = pos + 1;
        #[allow(clippy::never_loop)]
        'o: loop {
          for (pos2, c) in txt[p..].char_indices() {
            if c.is_ascii_digit() {
              continue;
            }
            if c == '.' {
              split!(p + pos2 + 1);
            }
            break 'o;
          }
          self.push_no_tran(txt);
          return;
        }
      } else if "-+".contains(i) {
        if let Some(c) = txt[pos + 1..].chars().next() {
          if c.is_whitespace() || ".-|:".contains(c) {
            split!(pos + 1 + c.len_utf8());
          }
        } else {
          self.push_no_tran(txt);
          return;
        }
      } else if '_' == i && txt[pos + 1..].chars().all(|c| c == i) {
        self.push_no_tran(txt);
        return;
      } else if i == '*' {
        if let Some(c) = txt[pos + 1..].chars().next() {
          if c == '*' {
            let t = &txt[pos + 2..];
            if t.chars().all(|k| k == '*') {
              self.push_no_tran(txt);
              return;
            } else {
              const END_FLAG: &str = "**";
              if let Some(p) = t.find(END_FLAG) {
                let p = p + 2;
                let tlen = t.len();
                if p <= tlen {
                  if p == tlen {
                    let begin = pos + 2;
                    tran_middle!(begin, txt_len - 2);
                  } else {
                    let mut len = 0;
                    for i in t[p..].chars() {
                      if i.is_whitespace() {
                        len += i.len_utf8();
                      } else if ":-".contains(i) {
                        len += i.len_utf8();
                        let p = p + pos + 2;
                        self.push_md(&txt[..p]);
                        let end = p + len;
                        self.push_no_tran(&txt[p..end]);
                        self.push_md(&txt[end..]);
                        return;
                      } else {
                        break;
                      }
                    }
                  }
                }
              }
            }
          } else {
            split!(pos + 1);
          }
        } else {
          self.push_no_tran(txt);
          return;
        }
      } else if i == '[' {
        let mut len = 0;
        let t = &txt[pos + 1..];
        for i in t.chars() {
          if i.is_whitespace() || i == 'x' {
            len += i.len_utf8();
          } else if i == ']' {
            len += 1;
            split!(pos + 1 + len);
          } else {
            break;
          }
        }
        for (pos2, c) in t.char_indices() {
          if c.is_ascii_digit() {
            continue;
          }
          if c == ']' {
            split!(pos2 + pos + 2);
          }
          break;
        }
        let mut iter = t.chars();
        while let Some(c) = iter.next() {
          if c.is_whitespace() {
            break;
          }
          if c == ']' {
            if let Some(next) = iter.next()
              && next == ':'
            {
              self.push_no_tran(txt);
              return;
            }
            break;
          }
        }
      } else if let Some(remain) = txt.strip_prefix("(")
        && let Some(p) = remain.find(")")
        && p + 1 == remain.len()
      {
        let p = pos + 1;
        self.push_no_tran(&txt[..p]);
        self.push_md(&txt[p..txt_len - 1]);
        self.push_no_tran(")");
        return;
      }
    }

    self.push_md_trim_start_line(txt);
  }
}
