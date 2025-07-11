use std::{collections::HashMap, fs::File, io::Read, path::Path};

use aok::Result;
use base64::prelude::{BASE64_STANDARD as B64, Engine};
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PackageJson {
  pub version: String,
  pub description: Option<String>,
  // Add other necessary fields here
}

#[derive(Serialize, Debug)]
pub struct Dist {
  pub tarball: String,
  pub shasum: String,
}

#[derive(Serialize, Debug)]
pub struct VersionData {
  pub name: String,
  pub version: String,
  pub dist: Dist,
}

#[derive(Serialize, Debug)]
pub struct Attachments {
  #[serde(rename = "content-type")]
  pub content_type: String,
  pub data: String,
  pub length: usize,
}

#[derive(Serialize, Debug)]
pub struct Payload {
  pub _id: String,
  pub name: String,
  pub access: String,
  #[serde(rename = "dist-tags")]
  pub dist_tags: HashMap<String, String>,
  pub versions: HashMap<String, VersionData>,
  pub _attachments: HashMap<String, Attachments>,
}

pub fn payload(pkg_name: &str, package_json: &Path, tgz: &Path) -> Result<Payload> {
  let json = std::fs::read(package_json)?;
  let package_json: PackageJson = sonic_rs::from_slice(&json)?;

  // Read the tarball file
  let mut file = File::open(tgz)?;
  let mut tarbuffer = Vec::new();
  file.read_to_end(&mut tarbuffer)?;

  // Base64 encode the tarball data
  let tar_data_base64 = B64.encode(&tarbuffer);
  let tar_length = tarbuffer.len();

  // Calculate SHA1 checksum
  let mut hasher = Sha1::new();
  hasher.update(&tarbuffer);
  let shasum = format!("{:x}", hasher.finalize());

  // Prepare package data
  let mut dist_tags = HashMap::new();
  dist_tags.insert("latest".to_string(), package_json.version.clone());

  let mut versions = HashMap::new();
  let dist = Dist {
    tarball: format!(
      "https://registry.npmjs.org/{}/-/{}-{}.tgz",
      pkg_name, pkg_name, package_json.version
    ),
    shasum,
  };
  let version_data = VersionData {
    name: pkg_name.into(),
    version: package_json.version.clone(),
    dist,
  };
  versions.insert(package_json.version.clone(), version_data);

  let mut attachments = HashMap::new();
  attachments.insert(
    format!("{}-{}.tgz", pkg_name, package_json.version),
    Attachments {
      content_type: "application/octet-stream".to_string(),
      data: tar_data_base64,
      length: tar_length,
    },
  );

  let package_data = Payload {
    _id: pkg_name.into(),
    access: "public".into(),
    name: pkg_name.into(),
    dist_tags,
    versions,
    _attachments: attachments,
  };

  Ok(package_data)
}
