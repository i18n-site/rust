use unicode_segmentation::UnicodeSegmentation;

const START_TAG: &str = "<code v=\"";
const START_TAG_LEN: usize = START_TAG.len();
const END_TAG: &str = "</code>";
const END_TAG_LEN: usize = END_TAG.len();

// 判断是否需要插入空格
pub fn word_push(pre: &mut String, txt: impl Into<String>) {
  let txt = txt.into();
  if let Some(last) = pre.chars().last()
    && let Some(first) = txt.chars().next()
  {
    let t = last.to_string() + &first.to_string();
    if !t.contains('_') {
      let mut t = t.split_word_bounds();
      if let (Some(_), None) = (t.next(), t.next()) {
        pre.push(' ')
      };
    }
  }
  pre.push_str(&txt);
}

/// replacer(前文, 原文, code_id)
pub fn code_replace(txt: impl AsRef<str>, replacer: impl Fn(&mut String, &str, &str)) -> String {
  let mut result = String::new();
  let mut txt = txt.as_ref();

  while let Some(start_tag) = txt.find(START_TAG) {
    word_push(&mut result, &txt[..start_tag]);

    let txt2 = &txt[start_tag..];
    if let Some(pos) = txt2[START_TAG_LEN..].find("\">") {
      let end = START_TAG_LEN + pos;
      let val = &txt2[START_TAG_LEN..end];
      let begin = end + 2;
      if let Some(pos) = txt2[begin..].find(END_TAG) {
        let offset = begin + pos + END_TAG_LEN;
        let org = &txt[start_tag..offset + start_tag];
        replacer(&mut result, org, val);
        txt = &txt2[offset..];
        continue;
      }
    }

    result.push_str(&txt[..START_TAG_LEN]);
    txt = &txt[START_TAG_LEN..];
  }

  result.push_str(txt);
  result
}
