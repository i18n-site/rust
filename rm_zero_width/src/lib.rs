pub fn rm_zero_width(s: impl AsRef<str>) -> String {
  s.as_ref()
    .chars()
    .filter(|&c| {
      !matches!(
        c,
        '\u{200B}' | '\u{200C}' | '\u{200D}' | '\u{200E}' | '\u{200F}' | '\u{FEFF}'
      )
    })
    .collect()
}
