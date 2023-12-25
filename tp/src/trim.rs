use daachorse::{CharwiseDoubleArrayAhoCorasick, CharwiseDoubleArrayAhoCorasickBuilder, MatchKind};

#[static_init::dynamic]
pub static ENTER: CharwiseDoubleArrayAhoCorasick<usize> =
  CharwiseDoubleArrayAhoCorasickBuilder::new()
    .match_kind(MatchKind::LeftmostFirst)
    .build(["\r\n", "\r", "\n"])
    .unwrap();

pub fn txt(md: impl AsRef<str>) -> String {
  let mut s = String::new();
  let mut pre = 0;
  let md = md.as_ref();
  for i in ENTER.leftmost_find_iter(md) {
    let start = i.start();
    let end = i.end();
    s.push_str(md[pre..start].trim_end());
    s.push_str(&md[start..end]);
    pre = end;
  }
  s.push_str(md[pre..].trim_end());
  s
}

pub fn push_line<'a>(mut line: &'a str, txt_li: &mut Vec<&'a str>, pos_li: &mut Vec<usize>) {
  let start = crate::pos::trim_start(line, |i| i.is_whitespace() || i == '*');

  if start > 0 {
    txt_li.push(&line[..start]);
    line = &line[start..];
  }

  let end = line.trim_end().len();
  if end > 0 {
    pos_li.push(txt_li.len());
    txt_li.push(&line[..end]);
  }

  if end != line.len() {
    txt_li.push(&line[end..]);
  }
}

pub fn push<'a>(md: &'a str, txt_li: &mut Vec<&'a str>, pos_li: &mut Vec<usize>) {
  let mut pre = 0;
  for i in ENTER.leftmost_find_iter(md) {
    let start = i.start();
    let end = i.end();

    push_line(&md[pre..start], txt_li, pos_li);
    txt_li.push(&md[start..end]);
    pre = end;
  }
  let end = md.len();
  if pre != end {
    push_line(&md[pre..end], txt_li, pos_li);
  }
}
