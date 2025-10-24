pub fn is_escape(md: &str) -> bool {
  let mut n = 0;
  let mut iter = md.chars().rev();
  while let Some('\\') = iter.next() {
    n += 1;
  }
  n % 2 != 0
}
