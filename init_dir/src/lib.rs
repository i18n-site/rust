pub fn ignore(cache: &std::path::Path) -> Result<(), std::io::Error> {
  if let Ok(meta) = std::fs::metadata(cache) {
    if meta.is_dir() {
      return Ok(());
    }
    std::fs::remove_file(cache)?;
  }
  std::fs::create_dir_all(cache)?;
  use std::io::Write;

  ifs::w(cache.join(".gitignore"))?.write_all(
    r#"**/*
!**/.gitignore"#
      .to_string()
      .as_bytes(),
  )?;
  Ok(())
}
