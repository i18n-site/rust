use std::{
  path::{Path, PathBuf},
  sync::Arc,
};

use aok::{OK, Result};
use aws_config::Region;
pub use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::{
  Client,
  config::{Credentials, SharedCredentialsProvider},
};

pub trait IntoByteStream {
  fn as_byte_stream(&self) -> impl std::future::Future<Output = Result<ByteStream>> + Send;
}

impl IntoByteStream for &Path {
  async fn as_byte_stream(&self) -> Result<ByteStream> {
    Ok(ByteStream::from_path(self).await?)
  }
}

impl IntoByteStream for &[u8] {
  async fn as_byte_stream(&self) -> Result<ByteStream> {
    Ok(ByteStream::from(bytes::Bytes::from(self.to_vec())))
  }
}

impl IntoByteStream for Vec<u8> {
  async fn as_byte_stream(&self) -> Result<ByteStream> {
    Ok(ByteStream::from(bytes::Bytes::from((self).clone())))
  }
}

impl IntoByteStream for &PathBuf {
  async fn as_byte_stream(&self) -> Result<ByteStream> {
    Ok(ByteStream::from_path(self).await?)
  }
}

impl IntoByteStream for PathBuf {
  async fn as_byte_stream(&self) -> Result<ByteStream> {
    Ok(ByteStream::from_path(self).await?)
  }
}

#[derive(Debug, Clone)]
pub struct _S3 {
  client: Client,
}

pub const 并发: usize = 32;
pub const MAX_RETRY: u8 = 3;

genv::def!(S3_REGION:String | "us-east-1".into());

impl _S3 {
  pub fn from_env() -> Self {
    Self::new(
      S3_REGION(),
      genv::get::<String>("S3_AK"),
      genv::get::<String>("S3_SK"),
    )
  }

  pub fn new(region: String, ak: String, sk: String) -> Self {
    let mut config = aws_config::SdkConfig::builder().region(Region::new(region));
    config.set_credentials_provider(Some(SharedCredentialsProvider::new(Credentials::new(
      ak, sk, None, //session_token: Option<String>,
      None, //expires_after: Option<SystemTime>,
      "",
    ))));
    config.set_behavior_version(Some(aws_config::BehaviorVersion::latest()));

    if let Ok(url) = std::env::var("S3_ENDPOINT") {
      config = config.endpoint_url(url);
    }

    let config = config.build();

    _S3 {
      client: Client::new(&config),
    }
  }

  async fn _put(
    &self,
    bucket: &String,
    key: &String,
    mime_type: &String,
    data: ByteStream,
  ) -> Result<()> {
    self
      .client
      .put_object()
      .bucket(bucket)
      .content_type(mime_type)
      .key(key)
      .body(data)
      .send()
      .await?;

    OK
  }

  pub async fn put(
    &self,
    bucket: impl Into<String>,
    key: impl Into<String>,
    mime_type: impl Into<String>,
    data: impl IntoByteStream,
  ) -> Result<()> {
    let bucket = bucket.into();
    let key = key.into();
    let mime_type = mime_type.into();

    let mut retry = 0;
    loop {
      let data = data.as_byte_stream().await?;
      match self._put(&bucket, &key, &mime_type, data).await {
        Ok(_) => return OK,
        Err(err) => {
          let prefix = format!("S3 {bucket}/{key} put");
          if retry == MAX_RETRY {
            tracing::error!("{prefix} failed : {err}");
            return Err(err);
          }
          retry += 1;
          tracing::warn!("{prefix} : {err} ( {retry} TIMES ) , WILL RETRY");
        }
      }
    }
  }
}

#[derive(Debug)]
pub struct S3Bucket {
  s3: _S3,
  bucket: String,
}

impl S3Bucket {
  pub fn from_env(bucket: impl Into<String>) -> Self {
    S3Bucket {
      s3: _S3::from_env(),
      bucket: bucket.into(),
    }
  }

  pub async fn put(
    &self,
    key: impl Into<String>,
    mime_type: impl Into<String>,
    data: impl IntoByteStream,
  ) -> Result<()> {
    self.s3.put(&self.bucket, key, mime_type, data).await
  }
}

pub fn from_env() -> S3Bucket {
  S3Bucket::from_env(genv::get::<String>("S3_BUCKET"))
}

#[derive(Clone)]
pub struct S3(Option<Arc<S3Bucket>>);

impl Default for S3 {
  fn default() -> Self {
    Self::new()
  }
}

impl S3 {
  pub fn new() -> Self {
    Self(None)
  }

  pub fn get(&mut self) -> Arc<S3Bucket> {
    if let Some(ref arc) = self.0 {
      return arc.clone();
    }
    let arc = Arc::new(from_env());
    self.0 = Some(arc.clone());
    arc
  }
}
