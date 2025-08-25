use std::future;

use faststr::FastStr;
use i18n_gateway::{
  cert_loader::{CertLoader, CertStrDb},
  error::Result,
};

struct FileCertDb;

impl CertStrDb for FileCertDb {
  fn get(
    &self,
    host: impl Into<FastStr>,
  ) -> impl future::Future<Output = Result<Option<(String, String)>>> + Send + Sync {
    let host_str: FastStr = host.into();
    async move {
      if host_str == "018007.xyz" {
        let cert_path = "examples/ssl/018007.xyz_ecc/fullchain.cer";
        let key_path = "examples/ssl/018007.xyz_ecc/018007.xyz.key";
        println!("load cert: {}", cert_path);
        let cert_pem = tokio::fs::read_to_string(cert_path).await.unwrap();
        let key_pem = tokio::fs::read_to_string(key_path).await.unwrap();
        return Ok(Some((cert_pem, key_pem)));
      }
      Ok(None)
    }
  }
}

#[tokio::main]
async fn main() {
  let db = FileCertDb;
  let cert_loader = CertLoader::new(db);

  // First time, load from db
  let cert = cert_loader.get("018007.xyz").await.unwrap();
  assert!(cert.is_some());

  // Second time, load from cache
  let cert = cert_loader.get("018007.xyz").await.unwrap();
  assert!(cert.is_some());

  // Remove the certificate by faking a long expiration check
  cert_loader.rm_expired(999999);

  // Third time, it should be loaded from db again because it was removed from cache
  let cert = cert_loader.get("018007.xyz").await.unwrap();
  assert!(cert.is_some());
}
