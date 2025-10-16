/*
输入html和tag,返回此tag的闭合标签位置。
注意: tag可能有嵌套，比如<code>abc<code>test</code>123</code>, 需要找到对应的闭合返回。
参数 htm_li 是迭代器，usize代表了偏移量，&str是行内容。
返回值为行的偏移量+闭合标记结尾的位置。
如果找不到闭合位置,返回0。
用状态机实现，标签可能不规范，比如 </ pre >
*/

#[derive(Debug)]
pub struct FindClose<'a> {
  pub stack: usize,
  pub tag: &'a str,
}

impl<'a> FindClose<'a> {
  pub fn new(tag: &'a str) -> Self {
    Self { stack: 0, tag }
  }

  pub fn find(&mut self, htm: impl AsRef<str>) -> Option<usize> {
    let htm = htm.as_ref();
    let tag = self.tag;

    if tag == "br" {
      return htm.find(">").map(|i| i + 1);
    }

    let htm = htm.to_lowercase();
    let mut iter = htm.char_indices();
    while let Some((_, c)) = iter.next() {
      if c == '<' {
        while let Some((_, c)) = iter.next() {
          if c.is_whitespace() {
            continue;
          }
          if c == '/' {
            let mut t = String::new();
            for (pos, c) in iter.by_ref() {
              if c == '>' {
                if t.trim() == tag {
                  if self.stack == 0 {
                    return Some(pos + 1);
                  }
                  self.stack -= 1;
                }
                break;
              }
              t.push(c);
            }
          } else {
            let mut t = String::from(c);
            for (_, c) in iter.by_ref() {
              if c == '>' {
                if t.trim() == tag {
                  self.stack += 1;
                }
                break;
              }
              t.push(c);
            }
          }
          break;
        }
      }
    }

    // 没有找到匹配的闭合标签
    None
  }
}
