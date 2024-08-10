use daachorse::{CharwiseDoubleArrayAhoCorasick, CharwiseDoubleArrayAhoCorasickBuilder, MatchKind};
use roaring::RoaringTreemap;

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

/// 将文本按：开头的标记（如 空白、*），正文内容， 结尾的空白字符，进行拆分
/// - `mut line: &'a str`：需要处理的原始字符串行。
/// - `txt_li: &mut Vec<&'a str>`：存储处理后文本段
/// - `pos_li: &mut Vec<usize>`：记录 `txt_li` 中非空白字符的位置
pub fn push_line<'a>(mut line: &'a str, txt_li: &mut Vec<&'a str>, pos_li: &mut RoaringTreemap) {
  // 如果存在开头的空白或星号字符，则将它们作为一个单独的文本片段添加 txt_li
  let start = crate::pos::trim_start(line, |i| i.is_whitespace() || i == '*');

  if start > 0 {
    txt_li.push(&line[..start]);
    line = &line[start..];
  }

  // 计算行尾部被去除的空白字符的长度。
  let end = line.trim_end().len();

  // 将非空白部分作为一个文本片段添加到 txt_li 中，并记录其位置到 pos_li
  if end > 0 {
    pos_li.push(txt_li.len() as u64);
    txt_li.push(&line[..end]);
  }

  // 将结尾空白字符，添加到列表中
  if end != line.len() {
    txt_li.push(&line[end..]);
  }
}

pub fn push<'a>(md: &'a str, txt_li: &mut Vec<&'a str>, pos_li: &mut RoaringTreemap) {
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
