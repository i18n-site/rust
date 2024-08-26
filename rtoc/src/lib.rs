use std::io::BufRead;

pub fn r(fp: impl AsRef<std::path::Path>) -> aok::Result<Vec<String>> {
  let mut r = vec![];
  for line in std::io::BufReader::new(std::fs::File::open(fp)?).lines() {
    if let Ok(line) = line {
      let line = line.trim();
      if line.starts_with("#") {
        continue;
      }
      r.push(line.into());
    }
  }
  Ok(r)
}
