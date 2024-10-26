use pos_lines::PosLines;

#[derive(Debug, PartialEq)]
pub enum Kind {
  Txt,
  Code,
  InlineCode,
  Br,
}

#[derive(Debug)]
pub struct Md<'a> {
  pub kind: Kind,
  pub str: &'a str,
}

pub fn md_parse<'a>(md: &'a str) -> Vec<Md<'a>> {
  let mut result = Vec::new();
  let mut start = 0;
  let mut in_code = false;
  let mut in_inline_code = false;
  let mut 代码开始位置 = 0;

  if md.is_empty() {
    return result;
  }

  if md.trim().is_empty() {
    result.push(Md {
      kind: Kind::Txt,
      str: md,
    });
    return result;
  }

  let mut line_iter = PosLines::new(md);
  let mut prev_end = 0;

  while let Some((行开始位置, line)) = line_iter.next() {
    dbg!(&line);
    // 在非代码块状态下处理换行符
    if !in_code && 行开始位置 > prev_end {
      result.push(Md {
        kind: Kind::Br,
        str: &md[prev_end..行开始位置],
      });
    }

    let line_end = 行开始位置 + line.len();
    let trimmed = line.trim();

    // 检查代码块开始/结束
    if trimmed == "```" {
      if !in_code {
        if start < 行开始位置 {
          let text = &md[start..行开始位置];
          if !text.trim().is_empty() {
            result.push(Md {
              kind: Kind::Txt,
              str: text,
            });
          }
        }
        in_code = true;
        代码开始位置 = 行开始位置;
        start = line_end;
      } else {
        let 代码块内容 = &md[代码开始位置..line_end];
        result.push(Md {
          kind: Kind::Code,
          str: 代码块内容,
        });
        in_code = false;
        start = line_end;
        // 更新 prev_end，这样下一行的换行符就会被正确处理
        prev_end = line_end;
        continue; // 跳过本次循环末尾的 prev_end 更新
      }
    } else if !in_code {
      // 处理行内代码
      let mut 字符迭代器 = line.char_indices().peekable();
      let mut found_backtick = false;

      while let Some((字符位置, c)) = 字符迭代器.next() {
        if c == '`' {
          let 绝对位置 = 行开始位置 + 字符位置;

          if !in_inline_code {
            if start < 绝对位置 {
              let text = &md[start..绝对位置];
              result.push(Md {
                kind: Kind::Txt,
                str: text,
              });
            }
            in_inline_code = true;
            found_backtick = true;
            代码开始位置 = 绝对位置;
            start = 绝对位置 + 1;
          } else {
            let 行内代码内容 = &md[代码开始位置..绝对位置 + 1];
            result.push(Md {
              kind: Kind::InlineCode,
              str: 行内代码内容,
            });
            in_inline_code = false;
            found_backtick = false;
            start = 绝对位置 + 1;
          }
        }
      }

      if found_backtick && in_inline_code {
        in_inline_code = false;
        start = 代码开始位置;
      }

      if start < line_end {
        let text = &md[start..line_end];
        if !text.is_empty() {
          result.push(Md {
            kind: Kind::Txt,
            str: text,
          });
        }
      }
      start = line_end;
    }

    prev_end = line_end;
  }

  // 处理最后剩余的文本
  if start < md.len() {
    let 剩余文本 = &md[start..];
    if in_code {
      result.push(Md {
        kind: Kind::Code,
        str: &md[代码开始位置..],
      });
    } else if !剩余文本.is_empty() {
      result.push(Md {
        kind: Kind::Txt,
        str: 剩余文本,
      });
    }
  }

  result
}
