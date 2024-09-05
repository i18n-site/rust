use std::{
  env, fs,
  io::Write,
  path::{Path, PathBuf},
};

use aok::Result;
use serde::{Deserialize, Serialize};
use sonic_rs::{from_str, to_string, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageJson {
  pub name: String,
}

pub struct Meta {
  pub fp: PathBuf,
  pub name: String,
}

pub fn package_json_ver(path: impl AsRef<Path>, ver: &str) -> Result<Meta> {
  let content = fs::read_to_string(path)?;
  let pkg = sonic_rs::from_str::<PackageJson>(&content)?;
  let mut package_json: std::collections::HashMap<String, Value> = from_str(&content)?;
  package_json.insert("version".into(), ver.into());
  let updated_content = to_string(&package_json)?;
  let temp_dir = env::temp_dir();
  let fp = temp_dir.join("package.json");
  let mut file = fs::File::create(&fp)?;
  file.write_all(updated_content.as_bytes())?;
  Ok(Meta { fp, name: pkg.name })
}
