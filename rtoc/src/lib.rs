use std::io::BufRead;

pub fn r(fp: impl AsRef<std::path::Path>) -> aok::Result<impl IntoIterator<Item = String>> {
  Ok(
    std::io::BufReader::new(std::fs::File::open(fp)?)
      .lines()
      .filter_map(|line| {
        if let Ok(line) = line {
          let line = line.trim();
          return if line.is_empty() || line.starts_with('#') {
            None
          } else {
            Some(line.into())
          };
        }
        None
      }),
  )
}
