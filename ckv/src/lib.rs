use aok::Void;

mod fs;
pub use fs::Fs;

mod s3;
pub use s3::S3;

pub trait Ckv {
  fn put(
    &self,
    rel: impl AsRef<str> + Send,
    bin: impl AsRef<[u8]> + Send,
  ) -> impl std::future::Future<Output = Void> + Send;

  fn put_path(
    &self,
    rel: impl AsRef<str> + Send,
    path: &str,
  ) -> impl std::future::Future<Output = Void> + Send;
}
