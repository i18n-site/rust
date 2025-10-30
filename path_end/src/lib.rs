pub fn path_end(dir: impl AsRef<str>) -> String {
  let dir = dir.as_ref();
  format!("{dir}{}", std::path::MAIN_SEPARATOR)
}
