use bytes::Bytes;

use crate::{Response, Result};

macro_rules! call {
  ($prefix: expr, $name:expr, $args:expr, $($await:tt)*) => {{
    use log::{error, info};

    let args = $args;
    let mut prefix = $prefix.into();
    prefix += " ";
    prefix += $name;

    let start = coarsetime::Instant::now();
    let result = Self::inner(args)$($await)*.into();
    let duration = start.elapsed().as_millis();
    prefix += &format!(" {duration}ms");

    match result {
      Result::Err(err) => {
          error!("{prefix} {args:?} {err}");
          Result::Response(Response {
          code: 500,
          body: err.to_string().into(),
          })
      }
      Result::Ok(r) => {
          info!("{prefix}");
          Result::Ok(r)
      }
      Result::Response(r) => {
          info!("{prefix} {}", r.code);
          Result::Response(r)
      }
    }
  }};
}

pub trait Call {
  type Args: std::fmt::Debug + TryFrom<Bytes>;
  type Result;

  fn inner(args: &Self::Args) -> impl Into<Result<Self::Result>>;

  fn call(prefix: impl Into<String>, name: &str, args: &Self::Args) -> Result<Self::Result> {
    call!(prefix, name, args,)
  }
}

// pub trait AsyncCall {
//   type Args: std::fmt::Debug + TryFrom<Bytes>;
//   type Result;
//
//   fn call(args: &Self::Args) -> impl Future<Output = Result<Self::Result>>;
//
//   fn call(
//     prefix: impl Into<String>,
//     name: &'static str,
//     args: Bytes,
//   ) -> impl std::future::Future<Output = Result<Self::Result>>
//   where
//     <<Self as AsyncCall>::Args as TryFrom<bytes::Bytes>>::Error: std::fmt::Display,
//   {
//     async move { call!(prefix, name, args, .await) }
//   }
// }
