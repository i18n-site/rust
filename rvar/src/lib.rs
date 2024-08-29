#[derive(Debug, Clone)]
pub struct RangeStr<'a> {
  pub str: Vec<&'a str>,
  pub range: Vec<std::ops::Range<usize>>,
}

impl<'a> RangeStr<'a> {
  pub fn replace<S: AsRef<str>>(&self, s: S, mut get: impl FnMut(&str) -> String) -> String {
    let s = s.as_ref();
    let mut result = String::new();
    let mut pre = 0;
    for (range, str) in self.range.iter().zip(self.str.iter()) {
      result.push_str(&s[pre..range.start]);
      let replacement = get(str);
      result.push_str(&replacement);
      pre = range.end;
    }
    result.push_str(&s[pre..]);
    result
  }
}

pub fn extract(input: &str) -> RangeStr<'_> {
  let bytes = input.as_bytes();
  let mut range_li = Vec::new();
  let mut str_li = Vec::new();
  let mut i = 0;

  while i < bytes.len() {
    if let Some(start) = find_subsequence(&bytes[i..], b"${") {
      let start_pos = i + start + 2;
      if let Some(end) = find_subsequence(&bytes[start_pos..], b"}") {
        let end_pos = start_pos + end;
        range_li.push(start_pos - 2..end_pos + 1);
        str_li.push(input[start_pos..=end_pos].trim());
        i = end_pos + 1;
      } else {
        // No closing brace found, stop further processing
        break;
      }
    } else {
      // No more "${" found, stop further processing
      break;
    }
  }

  RangeStr {
    range: range_li,
    str: str_li,
  }
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
  haystack
    .windows(needle.len())
    .position(|window| window == needle)
}
