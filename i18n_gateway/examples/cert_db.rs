use std::future::Future;

use faststr::FastStr;
use i18n_gateway::{
  cert_loader::CertStrDb,
  error::{Error, Result},
};

#[derive(Debug)]
pub struct FileCertDb;

impl CertStrDb for FileCertDb {
  fn get(
    &self,
    _host: impl Into<FastStr>,
  ) -> impl Future<Output = Result<Option<(String, String)>>> + Send + Sync {
    async {
      // 证书和私钥的路径
      let cert_path = "examples/ssl/018007.xyz_ecc/fullchain.cer";
      let key_path = "examples/ssl/018007.xyz_ecc/018007.xyz.key";

      // 异步读取文件内容
      let cert = tokio::fs::read_to_string(cert_path)
        .await
        .map_err(|e| Error::Io(e))?;
      let key = tokio::fs::read_to_string(key_path)
        .await
        .map_err(|e| Error::Io(e))?;

      Ok(Some((cert, key)))
    }
  }
}
