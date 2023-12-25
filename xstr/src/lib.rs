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

pub fn truncate(input: impl Into<String>, n: usize) -> String {
  let input = input.into();
  if input.len() > n {
    return input[..n].to_owned();
  }
  input
}

pub fn truncate255(input: impl Into<String>) -> String {
  truncate(input, 255)
}
