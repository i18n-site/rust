pub fn unix_path(path: impl Into<String>) -> String {
  let path = path.into();

  #[cfg(target_os = "windows")]
  {
    path.replace("\\", "/")
  }

  #[cfg(not(target_os = "windows"))]
  {
    path
  }
}
