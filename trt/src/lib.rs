use std::fmt::Display;

use static_init::dynamic;
pub use tokio::spawn;
use tokio::{macros::support::Future, runtime::Runtime};

#[dynamic]
pub static TRT: Runtime = tokio::runtime::Builder::new_multi_thread()
  .enable_all()
  .build()
  .unwrap();

pub fn bg<F, E>(future: F)
where
  E: Display,
  F: Future<Output = Result<(), E>> + Send + 'static,
{
  TRT.spawn(async move {
    if let Err(err) = future.await {
      tracing::error!("{err}");
    }
  });
}
