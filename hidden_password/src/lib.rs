pub fn hidden_password(s: impl AsRef<str>) -> String {
  let s = s.as_ref();
  if let Some(at_index) = s.rfind('@') {
    let password_start = s[..at_index].rfind(':').unwrap() + 1;
    let password_end = at_index;
    let mut result = s.to_string();
    result.replace_range(password_start..password_end, "***");
    result
  } else {
    s.into()
  }
}
