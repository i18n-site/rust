use std::{future::Future, pin::Pin};

use axum::{
  extract::{FromRequest, FromRequestParts, Request},
  handler::Handler,
  response::{IntoResponse, Response},
};
pub use bytes::Bytes;
pub use prost::Message;

pub trait ToResponse {
  fn to_response(self, req: &Request) -> Response;
}

pub struct Msg(pub Bytes);

impl<T: prost::Message> From<T> for Msg {
  default fn from(t: T) -> Self {
    let msg = t.encode_to_vec();
    Msg(if msg.is_empty() {
      [0][..].into()
    } else {
      msg.into()
    })
  }
}

macro_rules! impl_from {
($($ty:ty),*) => {
$(
    impl From<$ty> for Msg {
        fn from(t: $ty) -> Self {
            Msg(t.into())
        }
    }
)*
};
}

impl From<()> for Msg {
  fn from(_: ()) -> Self {
    Msg(Default::default())
  }
}

impl_from!(Bytes, Vec<u8>, String);

impl IntoResponse for Msg {
  fn into_response(self) -> Response {
    self.0.into_response()
  }
}

#[macro_export]
macro_rules! ok {
  ($expr:expr) => {{
    let t: $crate::Msg = $expr.into();
    Ok(t)
  }};
}

#[macro_export]
macro_rules! msg {
()=>{
$crate::Result<impl Into<$crate::Msg>>
}
}

#[derive(Clone)]
pub struct FnAny<F>(pub F);

pub type Result<T> = crate::Result<T, crate::Err>;

// pub fn into_response(result: Result<impl Into<Any>>) -> Response {
//   match result {
//     Ok(r) => r.into().into_response(),
//     Err(err) => err.into_response(),
//   }
// }

pub async fn await_into_response(
  result: impl Future<Output = Result<impl Into<Msg>>> + Send,
) -> Response {
  match result.await {
    Ok(r) => r.into().into_response(),
    Err(err) => err.into_response(),
  }
}

impl<F, Fut, S, T: Into<Msg>> Handler<((),), S> for FnAny<F>
where
  F: FnOnce() -> Fut + Clone + Send + 'static,
  Fut: Future<Output = Result<T>> + Send,
{
  type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

  fn call(self, _req: Request, _state: S) -> Self::Future {
    Box::pin(async move { await_into_response(self.0()).await })
  }
}

macro_rules! impl_handler {
(
[$($ty:ident),*], $last:ident
) => {

#[allow(non_snake_case, unused_mut)]
impl<F, Fut, S, M, T: Into<Msg>, $($ty,)* $last> Handler<(M, $($ty,)* $last,), S> for FnAny<F>
where
F: FnOnce($($ty,)* $last,) -> Fut + Clone + Send + 'static,
Fut: Future<Output = Result<T>> + Send,
S: Send + Sync + 'static,
$( $ty: FromRequestParts<S> + Send, )*
$last: FromRequest<S, M> + Send,
{
type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

fn call(self, req: Request, state: S) -> Self::Future {
Box::pin(async move {
let (mut parts, body) = req.into_parts();
let state = &state;

$(
    let $ty = match $ty::from_request_parts(
        &mut parts,
        state
    ).await {
        Ok(value) => value,
        Err(rejection) => return rejection.into_response(),
    };
)*

    let req = Request::from_parts(parts, body);

    let $last = match $last::from_request(req, state).await {
        Ok(value) => value,
        Err(rejection) => return rejection.into_response(),
    };

    await_into_response(self.0($($ty,)* $last,)).await

})
}
}
};
}

macro_rules! all_the_tuples {
  ($name:ident) => {
    $name!([], T1);
    $name!([T1], T2);
    $name!([T1, T2], T3);
    $name!([T1, T2, T3], T4);
    $name!([T1, T2, T3, T4], T5);
    $name!([T1, T2, T3, T4, T5], T6);
    $name!([T1, T2, T3, T4, T5, T6], T7);
    $name!([T1, T2, T3, T4, T5, T6, T7], T8);
    $name!([T1, T2, T3, T4, T5, T6, T7, T8], T9);
    $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9], T10);
    $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10], T11);
    $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11], T12);
    $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12], T13);
    $name!(
      [T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13],
      T14
    );
    $name!(
      [T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14],
      T15
    );
    $name!(
      [T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15],
      T16
    );
  };
}

all_the_tuples!(impl_handler);
