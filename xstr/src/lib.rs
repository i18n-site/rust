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

pub fn cut255(s: &str) -> &str {
  cut(s, 255)
}

pub fn cut(s: &str, max_length: usize) -> &str {
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

pub trait Join {
  fn join(self, split: impl AsRef<str>) -> String;
}

impl<I: IntoIterator<Item = S>, S: std::string::ToString> Join for I {
  fn join(self, split: impl AsRef<str>) -> String {
    let mut r = String::new();
    let split = split.as_ref();
    for i in self {
      r.push_str(&i.to_string());
      r.push_str(split);
    }
    if r.is_empty() {
      return String::new();
    }
    r[..r.len() - split.len()].into()
  }
}
