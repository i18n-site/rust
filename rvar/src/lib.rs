#[derive(Debug, Clone)]
pub struct RangeStr<'a> {
  pub str: Vec<&'a str>,
  pub range: Vec<std::ops::Range<usize>>,
}

pub fn replace<S: AsRef<str>>(
  s: S,
  li: &[std::ops::Range<usize>],
  mut get: impl FnMut(&str) -> String,
) -> String {
  let s = s.as_ref();
  let mut result = String::new();
  let mut pre = 0;
  for range in li {
    result.push_str(&s[pre..range.start]);
    let replacement = get(s[range.clone()].trim());
    result.push_str(&replacement);
    pre = range.end;
  }
  result.push_str(&s[pre..]);
  result
}

pub fn extract(input: &str) -> RangeStr<'_> {
  let bytes = input.as_bytes();
  let mut range_li = Vec::new();
  let mut str_li = Vec::new();
  let mut i = 0;

  while i < bytes.len() {
    if let Some(start) = find_subsequence(&bytes[i..], b"${") {
      let start_pos = i + start + 2;
      let mut j = start_pos;
      j = skip_whitespace(bytes, j);
      if let Some(end) = find_subsequence(&bytes[j..], b"}") {
        let end_pos = j + end;
        let mut word_end = end_pos - 1;
        while word_end > j {
          if b" \t".contains(&bytes[word_end]) {
            word_end -= 1;
          } else {
            break;
          }
        }

        range_li.push(start_pos..end_pos);
        str_li.push(&input[j..=word_end]);
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

fn skip_whitespace(bytes: &[u8], mut pos: usize) -> usize {
  while pos < bytes.len()
    && (bytes[pos] == b' ' || bytes[pos] == b'\t' || bytes[pos] == b'\n' || bytes[pos] == b'\r')
  {
    pos += 1;
  }
  pos
}
