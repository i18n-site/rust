pub fn is转义(line: impl AsRef<str>) -> bool {
  let mut n = 0;
  let line = line.as_ref();
  for c in line.chars().rev() {
    if c == '\\' {
      n += 1;
    } else {
      break;
    }
  }
  n % 2 == 1
}

pub fn whitespace_or_quote(c: char) -> bool {
  c.is_whitespace() || "\"'".contains(c)
}

pub fn fmt(md: impl AsRef<str>) -> String {
  md.as_ref()
    .trim_end()
    // 把 \r\n 和 \r 都变为 \n
    .lines()
    .map(|l| l.trim_end())
    .collect::<Vec<_>>()
    .join("\n")
}
