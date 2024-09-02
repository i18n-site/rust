#![feature(let_chains)]

use aok::Null;

mod fs;
pub use fs::Fs;

mod s3;
pub use s3::S3;

pub trait Ckv {
  fn put(
    &self,
    rel: impl AsRef<str> + Send,
    bin: impl AsRef<[u8]> + Send,
  ) -> impl std::future::Future<Output = Null> + Send;
}
