use std::{
  fs::File,
  io::{BufRead, BufReader},
  path::Path,
};

pub fn set(
  path: impl AsRef<Path>,
  key: impl AsRef<str>,
  value: impl AsRef<str>,
) -> std::io::Result<Vec<(String, String)>> {
  let path = path.as_ref();
  let value = value.as_ref();
  let key = key.as_ref();
  let mut result = vec![];
  let mut readed = vec![];
  let kv = (key.to_owned(), value.to_owned());
  if path.exists() {
    let mut changed = false;
    let mut has_key = false;
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines().map_while(Result::ok) {
      let line = line.trim();
      readed.push(line.to_owned());
      if line.starts_with("#") {
        continue;
      }
      if let Some(p) = line.find(":")
        && (p + 1) < line.len()
      {
        let k = line[0..p].trim();
        let v = line[p + 1..].trim();
        if k == key {
          has_key = true;
          result.push(kv.clone());
          if value != v {
            changed = true;
            readed.pop();
            readed.push(format!("{key}: {value}"));
          }
        } else {
          result.push((k.to_owned(), v.to_owned()));
        }
      }
    }
    if !has_key {
      result.insert(0, kv);
      changed = true;
    }
    if changed {
      ifs::wstr(path, readed.join("\n"))?;
    }
  } else {
    result.push(kv);
    ifs::wstr(path, format!("{key}: {value}"))?;
  }
  Ok(result)
}
