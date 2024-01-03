pub fn reverse(s: impl AsRef<str>) -> String {
  s.as_ref().chars().rev().collect()
}

pub fn lowtrim(input: impl AsRef<str>) -> String {
  input
    .as_ref()
    .to_lowercase()
    .chars()
    .filter(|&c| !c.is_whitespace())
    .collect()
}

pub fn is_ascii_digit(bytes: impl AsRef<[u8]>) -> bool {
  bytes.as_ref().iter().all(|i| {
    let i = *i;
    i.is_ascii_digit()
  })
}

pub fn word_reverse(input: &str, split: &str) -> String {
  let mut parts = Vec::new();
  let mut current_part = String::new();

  for c in input.chars() {
    if split.find(c).is_some() {
      parts.push(current_part);
      current_part = String::new();
      parts.push(c.to_string());
    } else {
      current_part.push(c);
    }
  }

  parts.push(current_part);
  parts.reverse();
  parts.concat()
}

// 这个实现有问题,可能切到不完整的utf8
// pub fn truncate(input: impl Into<String>, n: usize) -> String {
//   let input = input.into();
//   if input.len() > n {
//     return input[..n].to_owned();
//   }
//   input
// }
//
// pub fn truncate255(input: impl Into<String>) -> String {
//   truncate(input, 255)
// }

pub fn cut255<'a>(s: &'a str) -> &'a str {
  cut(s, 255)
}

pub fn cut<'a>(s: &'a str, max_length: usize) -> &'a str {
  if s.len() > max_length {
    let mut byte_count = 0;
    for c in s.chars() {
      let char_bytes = c.len_utf8();
      let t = byte_count + char_bytes;
      if t > max_length {
        break;
      }
      byte_count = t;
    }
    return &s[..byte_count];
  }
  s
}

pub fn join<S: std::string::ToString>(li: impl IntoIterator<Item = S>) -> String {
  li.into_iter()
    .map(|i| i.to_string())
    .collect::<Vec<String>>()
    .join(",")
}
