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
