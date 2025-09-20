use pos_lines::PosLines;

mod hugo;
pub mod kind;
mod mdli;
mod parser;
pub mod pos;

pub use kind::Kind;
pub use mdli::MdLi;
pub use parser::{fmt, is转义, whitespace_or_quote};

#[derive(Debug, Clone)]
pub struct Md {
  pub kind: Kind,
  pub str: String,
}

pub fn fmt_parse(md: impl AsRef<str>) -> MdLi {
  parse(fmt(md))
}

pub fn md_parse(mut md: &str, mdli: &mut MdLi) {
  let mut line_iter = PosLines::new(md);
  let mut prev_line_end = 0;

  'line: while let Some((mut md_pos, mut line)) = line_iter.next() {
    if md_pos > prev_line_end {
      mdli.push_break(Kind::Br, &md[prev_line_end..md_pos]);
    }

    prev_line_end = md_pos + line.len();

    let mut chars = line.char_indices();
    let mut offset = 0;

    'char: while let Some((mut line_pos, c)) = chars.next() {
      // dbg!((line_pos, c));
      // 当前行前面的字符
      macro_rules! push {
        () => {
          if line_pos > offset {
            mdli.push_txt(Kind::Txt, &line[offset..line_pos]);
          }
        };
      }

      macro_rules! truncate_line {
        ($len:expr) => {{
          md_pos += $len;
          line = &line[$len..];
          chars = line.char_indices();
          offset = 0;
        }};
      }

      macro_rules! push_kind {
        ($kind:expr, $end:expr) => {{
          push!();
          let end = $end;
          mdli.push($kind, &line[line_pos..end]);
          truncate_line!(end);
          continue 'char;
        }};
      }

      macro_rules! multi_line_push {
        ($kind:expr, $begin:expr, $end:expr) => {
          push!();
          mdli.push($kind, &md[$begin..$end]);
          md = md[$end..].into();
          line_iter = PosLines::new(&md);
          prev_line_end = 0;
          continue 'line;
        };
      }

      macro_rules! continue_if_转义 {
        () => {
          if is转义(&line[..line_pos]) {
            continue;
          }
        };
      }

      if line_pos == 0 {
        #[allow(clippy::never_loop)]
        'o1: loop {
          if c == '#' {
            let begin = line_pos + 1;
            let mut remain = line[begin..].char_indices();
            let end;
            let mut split_space_start = 0;
            #[allow(clippy::never_loop)]
            'o2: loop {
              while let Some((len, i)) = remain.next() {
                if i != '#' {
                  if !i.is_whitespace() {
                    break 'o1;
                  }
                  end = len + 1;
                  split_space_start = len + 2;
                  for (len, i) in remain.by_ref() {
                    if i.is_whitespace() {
                      split_space_start = len + 2;
                    } else {
                      break;
                    }
                  }
                  break 'o2;
                }
              }
              end = line.len();
              break;
            }
            mdli.push(Kind::H, &line[..end]);
            if split_space_start > 0 {
              mdli.push(Kind::Split, &line[end..split_space_start]);
              truncate_line!(split_space_start);
            } else {
              truncate_line!(end);
            }
          }
          break;
        }
      }

      // 后面还有字符才解析markdown结构
      if line.len() > line_pos + 1 {
        // 图片 & 链接
        if c == '[' {
          let begin = line_pos + 1;
          let mut remain = line[begin..].char_indices();
          while let Some((len, i)) = remain.next() {
            if i == '[' {
              break;
            }
            if i == ']'
              && let Some((_, i)) = remain.next()
            {
              let txt_end_pos = begin + len;
              if i == '(' {
                for (len, i) in remain.by_ref() {
                  if i == ')' {
                    let is_img = line[..line_pos].ends_with("!");
                    let txt_begin = &line[line_pos - if is_img { 1 } else { 0 }..line_pos + 1];
                    let txt = &line[line_pos + 1..txt_end_pos];
                    let txt_end = &line[txt_end_pos..txt_end_pos + 2];
                    let end = begin + len;
                    let url = &line[(txt_end_pos + 2)..end];
                    let end = end + 1;
                    let url_end = &line[begin + len..end];

                    if is_img {
                      // img 前面有一个!
                      line_pos -= 1;
                      push!();
                      mdli.push(Kind::ImgBegin, txt_begin);
                      mdli.push_txt(Kind::ImgTxt, txt);
                      mdli.push(Kind::ImgTxtEnd, txt_end);
                      mdli.push(Kind::Img, url);
                      mdli.push(Kind::ImgEnd, url_end);
                    } else {
                      push!();
                      mdli.push(Kind::UrlBegin, txt_begin);
                      mdli.push_txt(Kind::UrlTxt, txt);
                      mdli.push(Kind::UrlTxtEnd, txt_end);
                      mdli.push(Kind::Url, url);
                      mdli.push(Kind::UrlEnd, url_end);
                    };
                    truncate_line!(end);
                    continue 'char;
                  }
                }
              }
            }
          }
        } else if c == '`' {
          continue_if_转义!();
          let mut remain = &line[line_pos + 1..];
          if remain.starts_with("``") {
            // 多行代码
            let md_begin = md_pos + line_pos;
            let mut begin = md_begin + 3;
            remain = &md[begin..];
            while let Some(p) = remain.find("```") {
              if is转义(&remain[..p]) {
                let p = p + 3;
                begin += p;
                remain = &remain[p..];
                continue;
              }
              multi_line_push!(Kind::Code, md_begin, begin + p + 3);
            }
          } else {
            let mut begin = line_pos + 1;
            // 单行代码
            while let Some(p) = remain.find("`") {
              if remain[p..].starts_with("```")
                && let Some(last) = remain[..p].chars().last()
                && last.is_whitespace()
              {
                begin += 4;
                remain = &remain[p + 3..];
                continue;
              } else if is转义(&remain[..p]) {
                let p = p + 1;
                begin += p;
                remain = &remain[p..];
                continue;
              }
              push_kind!(Kind::Code, begin + p + 1);
            }
          }
        } else if c == '<' {
          let md_begin = md_pos + line_pos;
          // html 标签最少3个字符
          if md.len() > md_begin + 2 {
            let remain = &md[md_begin + 1..];
            if remain.starts_with("!--") {
              // html comment
              let begin = md_begin + 4;
              if let Some(len) = md[begin..].find("-->") {
                multi_line_push!(Kind::HtmComment, md_begin, begin + len + 3);
              }
            } else if let Some(remain) = remain.strip_prefix("/") {
              // html close
              let mut citer = remain.char_indices();
              while let Some((_, i)) = citer.next() {
                if i.is_whitespace() {
                  continue;
                }
                // 首字母是英文
                if i.is_ascii_alphabetic() {
                  let mut has_whitespace = false;
                  for (len, i) in citer.by_ref() {
                    if i.is_ascii_alphanumeric() {
                      // 闭合标签不能有多个单词
                      if has_whitespace {
                        break;
                      }
                      continue;
                    }
                    if i.is_whitespace() {
                      has_whitespace = true;
                      continue;
                    }
                    if i == '>' {
                      let end = line_pos + len + 3;
                      push_kind!(Kind::HtmClose, end);
                    } else {
                      break;
                    }
                  }
                } else {
                  break;
                }
              }
            } else {
              // html open
              for (_, i) in remain.char_indices() {
                if i.is_ascii_alphabetic() {
                  let begin = md_begin + 2;
                  for (len, i) in md[begin..].char_indices() {
                    if i == '<' {
                      break;
                    }
                    if i == '>' {
                      multi_line_push!(Kind::HtmOpen, md_begin, begin + len + 1);
                    }
                  }
                }
              }
            }
          }
        }
      }
    }

    // 这里 prev_line_end 的当前行
    if offset < line.len() {
      mdli.push_txt(Kind::Txt, &line[offset..]);
    }
  }

  if prev_line_end < md.len() {
    let line = &md[prev_line_end..];
    mdli.push(Kind::Br, line);
  }
}

pub fn parse(md: impl AsRef<str>) -> MdLi {
  let (md, mut mdli) = hugo::remove_head(md.as_ref());
  md_parse(md, &mut mdli);
  mdli
}
