#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;

use std::{
  env,
  fs::{self, File},
  io,
  io::{BufReader, Read, Write},
  path::{Path, PathBuf},
};

use ed25519_dalek::{Signature, VerifyingKey};
pub use error::{Error, Result};
use sha3::{Digest, Sha3_512};

pub fn check<P: AsRef<Path>>(
  version: impl AsRef<[u8]>,
  tar_path: P,
  pk: [u8; 32],
) -> Result<Option<PathBuf>> {
  let tar_path = tar_path.as_ref();
  let dir = env::temp_dir().join("upgradeVerify").join(
    tar_path
      .file_name()
      .map(|i| i.to_str())
      .unwrap_or(None)
      .unwrap_or("_"),
  );
  // 解压
  {
    if dir.exists() {
      fs::remove_dir_all(&dir)?;
    }
    fs::create_dir_all(&dir)?;

    let tar_file = File::open(tar_path)?;
    let mut archive = tar::Archive::new(tar_file);
    archive.unpack(&dir)?;
  }

  let tar_zst = dir.join("tar.zst");
  let sign = dir.join("sign");

  if !tar_zst.exists() || !sign.exists() {
    return Ok(None);
  }

  // 计算散列
  let file = File::open(tar_zst)?;
  let mut reader = BufReader::new(file);
  let mut hasher = Sha3_512::new();
  hasher.update(version.as_ref());
  let mut buf = [0u8; 8192];
  loop {
    let count = reader.read(&mut buf)?;
    if count == 0 {
      break;
    }
    hasher.update(&buf[..count]);
  }
  let sign = fs::read(sign)?;
  if let Ok(sign) = sign.try_into() {
    let public_key = VerifyingKey::from_bytes(&pk)?;
    let sign = Signature::from_bytes(&sign);
    public_key.verify_prehashed(hasher, None, &sign)?;
    return Ok(Some(dir));
  }
  Ok(None)
}
