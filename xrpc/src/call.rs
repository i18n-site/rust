use std::fmt::Debug;

use crate::{Response, Result};

macro_rules! call {
  ($prefix: expr, $args:expr, $($await:tt)*) => {{
    use log::{error, info};

    let args = $args;
    let mut prefix = $prefix.into();
    prefix += " ";
    prefix += Self::name();

    let start = coarsetime::Instant::now();
    let result = Self::inner(args)$($await)*;
    let duration = start.elapsed().as_millis();
    prefix += " ";
    prefix += &duration.to_string();
    prefix += "ms";

    match result {
      Result::Ok(r) => {
        info!("{prefix}");
        Result::Ok(r)
      }
      Result::Err(err) => {
        error!("{prefix} {args:?} {err}");
        Result::Response(Response {
          code: 500,
          body: err.to_string().into(),
        })
      }
      Result::Response(r) => {
        info!("{prefix} {}", r.code);
        Result::Response(r)
      }
    }
  }};
}

pub trait Call {
  type Args: Debug;
  type Result;

  fn inner(args: &Self::Args) -> Result<Self::Result>;

  fn name() -> &'static str;

  fn call(prefix: impl Into<String>, args: &Self::Args) -> Result<Self::Result> {
    call!(prefix, args,)
  }
}

pub trait AsyncCall {
  type Args: Debug;
  type Result;

  fn inner(args: &Self::Args) -> impl Future<Output = Result<Self::Result>>;

  fn name() -> &'static str;

  fn call(
    prefix: impl Into<String>,
    args: &Self::Args,
  ) -> impl std::future::Future<Output = Result<Self::Result>> {
    async move { call!(prefix, args, .await) }
  }
}
