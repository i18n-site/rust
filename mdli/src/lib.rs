use pos_lines::PosLines;

#[derive(Debug, PartialEq)]
pub enum Kind {
  Txt,
  Code,
  InlineCode,
  Br,
  EmptyLine,
}

#[derive(Debug)]
pub struct Md {
  pub kind: Kind,
  pub str: String,
}

#[derive(Debug)]
pub struct MdLi(pub Vec<Md>);

impl MdLi {
  pub fn join(&self) -> String {
    self.0.iter().map(|md| md.str.as_str()).collect()
  }

  pub fn push(&mut self, kind: Kind, str: impl Into<String>) {
    self.0.push(Md {
      kind,
      str: str.into(),
    });
  }
}

pub fn md_parse(md: impl AsRef<str>) -> MdLi {
  let md = md.as_ref();
  let mut result = MdLi(vec![]);
  let mut in_code = false;
  let mut 代码开始位置 = 0;

  if md.is_empty() {
    return result;
  }

  if md.trim().is_empty() {
    result.push(Kind::Txt, md);
    return result;
  }

  let line_iter = PosLines::new(md);
  let mut prev_end = 0;

  for (行开始位置, line) in line_iter {
    let not_in_code = !in_code;
    // 在非代码块状态下处理换行符
    if not_in_code && 行开始位置 > prev_end {
      result.push(Kind::Br, &md[prev_end..行开始位置]);
    }

    let trimmed = line.trim();

    let line_end = 行开始位置 + line.len();

    if trimmed.is_empty() {
      result.push(Kind::EmptyLine, line);

      prev_end = line_end;
      continue;
    }

    // 检查代码块开始/结束
    if trimmed.starts_with("```") {
      if !in_code {
        in_code = true;
        代码开始位置 = 行开始位置;
        prev_end = line_end;
      } else {
        let 代码块内容 = &md[代码开始位置..line_end];
        result.push(Kind::Code, 代码块内容);
        in_code = false;
        prev_end = line_end;
        continue;
      }
    } else if !in_code {
      // 处理行内代码和普通文本
      let mut last_end = 0;
      let mut chars = line.char_indices();

      while let Some((i, c)) = chars.next() {
        if c == '`' {
          // 检查是否有配对的反引号
          let mut found_end = false;
          let mut end_byte_pos = i;
          let mut temp_chars = chars.clone();

          while let Some((j, c)) = temp_chars.next() {
            if c == '`' {
              found_end = true;
              end_byte_pos = j;
              chars = temp_chars;
              break;
            }
          }

          if found_end {
            // 添加反引号前的文本
            if last_end < i {
              let text = &line[last_end..i];
              result.push(Kind::Txt, text);
            }

            // 添加行内代码
            result.push(Kind::InlineCode, &line[i..=end_byte_pos]);

            last_end = end_byte_pos + 1;
          }
        }
      }

      // 添加剩余的文本
      if last_end < line.len() {
        let text = &line[last_end..];
        if !text.is_empty() {
          result.push(Kind::Txt, text);
        }
      }

      prev_end = line_end;
    }
  }

  // 处理最后剩余的文本
  if prev_end < md.len() {
    let 剩余文本 = &md[prev_end..];
    if in_code {
      result.push(Kind::Code, &md[代码开始位置..]);
    } else if !剩余文本.is_empty() {
      result.push(Kind::Txt, 剩余文本);
    }
  }

  result
}
