use std::{borrow::Borrow, fmt::Debug};

use crate::{Ext, HeadersExt, Log, Map, ParseError, Response, Result};

macro_rules! call {
  ($req:expr, $args: expr, $($await:tt)*) => {{
    let req = $req.borrow();
    let name = Self::name();
    match ($args) {
      Ok(args)=>{
        let req_args = ReqArgs { req, args: &args };
        let start = coarsetime::Instant::now();
        let result = Self::inner(req_args)$($await)*;
        let cost = start.elapsed().as_millis();

        match result {
          Result::Ok(r) => {
            L::info(name, req, args, cost);
            Result::Ok(r)
          }
          Result::Err(err) => {
            L::error(name, req, args, cost, &err);
            Result::Response(Response {
              code: 500,
              body: err.to_string().into(),
            })
          }
          Result::Response(r) => {
            L::response(name, req, args, cost, &r);
            Result::Response(r)
          }
        }
      }
      Err(err)=>{
        L::args_parse(name, req, &err);
        Result::Response(Response {
          code: 500,
          body: err.to_string().into(),
        })
      }
   }
  }};
}

pub trait Func {
  type Args: Debug;
  type Result;

  fn name() -> &'static str;
}

pub struct ReqArgs<'a, H: Map, E: Ext, A> {
  pub req: &'a HeadersExt<H, E>,
  pub args: &'a A,
}

pub type ParseResult<T> = std::result::Result<T, ParseError>;

pub trait Call: Func {
  fn inner<H: Map, E: Ext>(req_args: ReqArgs<H, E, Self::Args>) -> Result<Self::Result>;

  fn call<L: Log, H: Map, E: Ext>(
    (req, args): (impl Borrow<HeadersExt<H, E>>, ParseResult<Self::Args>),
  ) -> Result<Self::Result> {
    call!(req, args,)
  }
}

pub trait AsyncCall: Func {
  fn inner<H: Map, E: Ext>(
    req_args: ReqArgs<H, E, Self::Args>,
  ) -> impl Future<Output = Result<Self::Result>>;

  fn call<L: Log, H: Map, E: Ext>(
    (req, args): (impl Borrow<HeadersExt<H, E>>, ParseResult<Self::Args>),
  ) -> impl std::future::Future<Output = Result<Self::Result>> {
    async move { call!(req, args, .await) }
  }
}
