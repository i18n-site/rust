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
                  return pos;
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

// #[cfg(test)]
// mod tests {
//   use super::*;
//   use crate::find_close::find_close;
//
//   #[test]
//   fn test_simple_tag() {
//     let html = vec![(0, "<div>Hello</div>")];
//     assert_eq!(find_close(html, "div"), 12);
//   }
//
//   #[test]
//   fn test_nested_tags() {
//     let html = vec![(0, "<code>abc<code>test</code>123</code>")];
//     assert_eq!(find_close(html, "code"), 31);
//   }
//
//   #[test]
//   fn test_multiline() {
//     let html = vec![
//       (0, "<pre>"),
//       (5, "  line 1"),
//       (15, "  line 2"),
//       (25, "</pre>"),
//     ];
//     assert_eq!(find_close(html, "pre"), 31);
//   }
//
//   #[test]
//   fn test_irregular_closing_tag() {
//     let html = vec![(0, "<div>Hello</ div >")];
//     assert_eq!(find_close(html, "div"), 15);
//   }
//
//   #[test]
//   fn test_case_insensitive() {
//     let html = vec![(0, "<DIV>Hello</div>")];
//     assert_eq!(find_close(html, "div"), 12);
//   }
//
//   #[test]
//   fn test_tag_not_found() {
//     let html = vec![(0, "<div>Hello</div>")];
//     assert_eq!(find_close(html, "span"), 0);
//   }
//
//   #[test]
//   fn test_unclosed_tag() {
//     let html = vec![(0, "<div>Hello")];
//     assert_eq!(find_close(html, "div"), 0);
//   }
//
//   #[test]
//   fn test_complex_nesting() {
//     let html = vec![
//       (0, "<div>"),
//       (5, "  <p>Paragraph 1</p>"),
//       (25, "  <div>"),
//       (32, "    <p>Nested paragraph</p>"),
//       (58, "  </div>"),
//       (66, "  <p>Paragraph 2</p>"),
//       (86, "</div>"),
//     ];
//     assert_eq!(find_close(html, "div"), 92);
//   }
//
//   #[test]
//   fn test_attributes() {
//     let html = vec![(0, "<div class=\"container\">Hello</div>")];
//     assert_eq!(find_close(html, "div"), 30);
//   }
//
//   #[test]
//   fn test_self_closing_tag() {
//     let html = vec![(0, "<div><img src=\"image.jpg\"/>Text</div>")];
//     assert_eq!(find_close(html, "div"), 35);
//   }
// }
