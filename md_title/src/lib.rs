pub fn title_trim(input: &str) -> &str {
  let mut chars = input.chars();
  while let Some(c) = chars.next() {
    if c.is_whitespace() {
      continue;
    } else if c == '#' {
      return chars.as_str().trim();
    } else {
      return "";
    }
  }
  ""
}

pub fn md_title(md: &str) -> &str {
  for i in md.lines() {
    let t = title_trim(i);
    if !t.is_empty() {
      return t;
    }
  }
  ""
}
