pub const I18N: &[u8] = b"I18N.";

#[derive(Debug, Clone)]
pub struct RangeStr<'a> {
  pub str: Vec<&'a str>,
  pub range: Vec<std::ops::Range<usize>>,
}

pub fn replace<S: AsRef<str>>(s: &str, li: &[std::ops::Range<usize>], to_li: &[S]) -> String {
  // assert_eq!(
  //   li.len(),
  //   to_li.len(),
  //   "Ranges and replacements must have the same length"
  // );

  let mut result = s.to_string();

  for (range, replacement) in li.iter().zip(to_li).rev() {
    result.replace_range(range.clone(), replacement.as_ref());
  }

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
      let word_begin = j + I18N.len();
      if bytes[j..].starts_with(I18N) {
        j += I18N.len(); // Move past "I18N."
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
          str_li.push(&input[word_begin..=word_end]);
          i = end_pos + 1;
        } else {
          // No closing brace found, stop further processing
          break;
        }
      } else {
        // No valid I18N. found, move past "${"
        i = start_pos + 2;
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
