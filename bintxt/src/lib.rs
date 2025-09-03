pub fn bintxt<S: AsRef<str>>(li: impl IntoIterator<Item = S>) -> Vec<u8> {
  let mut r = Vec::new();
  for i in li {
    r.extend(i.as_ref().as_bytes());
    r.push(0);
  }
  if !r.is_empty() {
    r.pop();
  }

  r
}
