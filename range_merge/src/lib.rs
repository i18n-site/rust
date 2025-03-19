use std::ops::Range;

pub fn merge<S: AsRef<str>>(
  txt: impl AsRef<str>,
  range_li: impl AsRef<[Range<usize>]>,
  replace_li: impl AsRef<[S]>,
) -> String {
  let txt = txt.as_ref();
  let mut r = vec![];
  let mut pre = 0;
  for (range, traned) in range_li.as_ref().iter().zip(replace_li.as_ref()) {
    if range.start != pre {
      r.push(&txt[pre..range.start]);
    }
    r.push(traned.as_ref());
    pre = range.end;
  }
  if pre != txt.len() {
    r.push(&txt[pre..]);
  }
  r.join("")
}
