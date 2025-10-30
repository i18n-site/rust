pub fn ext_name(path: &str) -> Option<&str> {
  let len = path.len();
  if len > 1
    && let Some(pos) = path[..len - 1].rfind('.')
  {
    return Some(&path[pos + 1..len]);
  }
  None
}
