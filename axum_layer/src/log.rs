use std::fmt::Debug;

use axum::{http::Request, response::Response};
use futures::{FutureExt, future::Map};
use tower::Service;

#[derive(Clone)]
pub struct Log;

impl<S> tower::Layer<S> for Log {
  type Service = ResponseTimeService<S>;

  fn layer(&self, service: S) -> Self::Service {
    ResponseTimeService { inner: service }
  }
}

#[derive(Clone)]
pub struct ResponseTimeService<S> {
  inner: S,
}

impl<S, B, T> Service<Request<B>> for ResponseTimeService<S>
where
  S::Response: Debug + axum::response::IntoResponse, // 约束为 IntoResponse，它可以转换为 Response
  S::Error: Debug,
  S: Service<Request<B>, Response = Response<T>>, // 约束 S::Response 为 Response<T>
  B: Send + Debug,
{
  type Response = S::Response;
  type Error = S::Error;
  type Future =
    Map<S::Future, impl FnOnce(Result<S::Response, S::Error>) -> Result<S::Response, S::Error>>;

  fn poll_ready(
    &mut self,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Result<(), Self::Error>> {
    self.inner.poll_ready(cx)
  }

  fn call(&mut self, req: Request<B>) -> Self::Future {
    // let cost = cost_time::start();
    let method = req.method().to_owned();
    let url = req.uri().to_owned();

    self.inner.call(req).map(move |response| {
      // #[cfg(not(feature = "stackdriver"))]
      {
        // tracing::error!(&response);
        println!(
          // "{method} {url} {} {}s",
          "{} {method} {url}",
          if let Ok(ref response) = response {
            response.status().as_u16()
          } else {
            500
          },
          // cost.sec(),
        )
      }
      response
    })
  }
}
