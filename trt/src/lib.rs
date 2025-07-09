#![feature(macro_metavar_expr)]

use std::fmt::Display;

use static_init::dynamic;
pub use tokio::{self, spawn as _spawn};
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

// pub fn spawn<F>(future: F) -> JoinHandle<F::Output> ⓘ
// where
//     F: Future + Send + 'static,
//     F::Output: Send

pub async fn spawn<R>(
  future: impl Future<Output = aok::Result<R>> + Send + 'static,
) -> aok::Result<R>
where
  R: Send + 'static,
{
  _spawn(future).await?
}

#[macro_export]
macro_rules! join {
  ($($i:expr),*$(,)?) => {{
    let r = $crate::tokio::join!($($i),*);
    (
      $(
        ${ignore($i)}
        r.${index()}?
      ),*
    )
  }};
}
