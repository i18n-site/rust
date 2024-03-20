use std::{
  fs,
  fs::{create_dir_all, read, File, OpenOptions},
  io::BufWriter,
  path::{Path, PathBuf},
};

pub fn ensure_file_dir(path: impl Into<PathBuf>) -> Result<(), std::io::Error> {
  let p: PathBuf = path.into();
  if let Some(p) = p.parent() {
    create_dir_all(p)?;
  }
  Ok(())
}

pub fn w(path: impl AsRef<Path>) -> Result<BufWriter<File>, tokio::io::Error> {
  ensure_file_dir(path.as_ref())?;
  Ok(BufWriter::new(File::create(path)?))
}

pub fn r(path: impl AsRef<Path>) -> Result<Vec<u8>, tokio::io::Error> {
  match read(path) {
    Ok(v) => Ok(v),
    Err(err) => {
      if err.kind() == std::io::ErrorKind::NotFound {
        Ok(Vec::new())
      } else {
        Err(err)
      }
    }
  }
}

pub fn append(path: impl AsRef<Path>) -> Result<BufWriter<File>, tokio::io::Error> {
  Ok(BufWriter::new(OpenOptions::new().append(true).open(path)?))
}

pub fn size(path: impl AsRef<Path>) -> u64 {
  if let Ok(meta) = std::fs::metadata(path) {
    meta.len()
  } else {
    0
  }
}

pub fn len_mtime(path: impl AsRef<Path>) -> Option<(u64, u64)> {
  use std::time::{Duration, UNIX_EPOCH};

  if let Ok(meta) = fs::metadata(&path) {
    let mtime = meta
      .modified()
      .map(|i| {
        i.duration_since(UNIX_EPOCH)
          .unwrap_or(Duration::from_secs(0))
          .as_millis()
      })
      .unwrap_or(0) as u64;
    return Some((meta.len(), mtime));
  }
  None
}
