use std::{
  fmt::Display,
  path::{Path, PathBuf},
  sync::Arc,
  time::Duration,
};

use tokio::fs::{create_dir_all, File, OpenOptions};

pub async fn ensure_dir(path: impl Into<PathBuf>) -> Result<(), tokio::io::Error> {
  let p: PathBuf = path.into();
  if let Some(p) = p.parent() {
    create_dir_all(p).await?;
  }
  Ok(())
}

pub async fn w(path: impl Into<PathBuf>) -> Result<File, tokio::io::Error> {
  let path = path.into();
  ensure_dir(&path).await?;
  File::create(path).await
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
