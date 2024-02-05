use std::path::{Path, PathBuf};

use tokio::fs::{create_dir_all, File, OpenOptions};

pub async fn ensure_file_dir(path: impl Into<PathBuf>) -> Result<(), tokio::io::Error> {
  let p: PathBuf = path.into();
  if let Some(p) = p.parent() {
    create_dir_all(p).await?;
  }
  Ok(())
}

pub async fn w(path: impl AsRef<Path>) -> Result<File, tokio::io::Error> {
  ensure_file_dir(path.as_ref()).await?;
  File::create(path).await
}

pub async fn r(path: impl AsRef<Path>) -> Result<Vec<u8>, tokio::io::Error> {
  match tokio::fs::read(path).await {
    Ok(v) => Ok(v),
    Err(err) => {
      if err.kind() == tokio::io::ErrorKind::NotFound {
        Ok(Vec::new())
      } else {
        Err(err)
      }
    }
  }
}

pub async fn append(path: impl AsRef<Path>) -> Result<File, tokio::io::Error> {
  OpenOptions::new().append(true).open(path).await
}

pub async fn size(path: impl AsRef<Path>) -> u64 {
  if let Ok(meta) = tokio::fs::metadata(path).await {
    meta.len()
  } else {
    0
  }
}
