/*
输入html和tag,返回此tag的闭合标签位置。
注意: tag可能有嵌套，比如<code>abc<code>test</code>123</code>, 需要找到对应的闭合返回。
参数 htm_li 是迭代器，usize代表了偏移量，&str是行内容。
返回值为行的偏移量+闭合标记结尾的位置。
如果找不到闭合位置,返回0。
用状态机实现，标签可能不规范，比如 </ pre >
*/

pub fn find_close(htm: &str, tag: impl AsRef<str>) -> usize {
  let tag = tag.as_ref();
  let mut stack = 0; // 用于跟踪嵌套层级

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
                if stack == 0 {
                  return pos + 1;
                }
                stack -= 1;
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
                stack += 1;
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
  0
}
