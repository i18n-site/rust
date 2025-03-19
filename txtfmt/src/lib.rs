pub fn txtfmt(txt: impl AsRef<str>) -> String {
  txt.as_ref()
    .trim_end()
    // 把 \r\n 和 \r 都变为 \n
    .lines()
    .map(|l| l.trim_end())
    .collect::<Vec<_>>()
    .join("\n")
}
