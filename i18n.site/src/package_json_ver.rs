use std::{
  env, fs,
  io::Write,
  path::{Path, PathBuf},
};

use aok::Result;
use sonic_rs::{from_str, to_string, Value};

pub fn package_json_ver(path: impl AsRef<Path>, ver: &str) -> Result<PathBuf> {
  let content = fs::read_to_string(path)?;
  let mut package_json: std::collections::HashMap<String, Value> = from_str(&content)?;
  package_json.insert("version".into(), ver.into());
  let updated_content = to_string(&package_json)?;
  let temp_dir = env::temp_dir();
  let output_path = temp_dir.join("package.json");
  let mut file = fs::File::create(&output_path)?;
  file.write_all(updated_content.as_bytes())?;

  Ok(output_path)
}
