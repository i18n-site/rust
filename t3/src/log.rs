use std::fmt::Debug;

use axum::http::Request;
use futures::{future::Map, FutureExt};
pub use loginit::init;
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

impl<S, B> Service<Request<B>> for ResponseTimeService<S>
where
  S::Response: Debug,
  S::Error: Debug,
  S: Service<Request<B>>,
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
    #[cfg(not(feature = "stackdriver"))]
    let start_time: u64 = coarsetime::Clock::now_since_epoch().as_millis();
    #[cfg(not(feature = "stackdriver"))]
    let method = req.method().to_owned();
    #[cfg(not(feature = "stackdriver"))]
    let url = req.uri().to_owned();
    // #[cfg(not(feature = "stackdriver"))]
    // tracing::info!("{method} {url}");

    // #[cfg(feature = "stackdriver")]
    // let user_agent;
    // #[cfg(feature = "stackdriver")]
    // {
    //   let headers = req.headers();
    //   user_agent = match headers.get("user-agent") {
    //     Some(t) => Some(t.to_str().unwrap().to_owned()),
    //     None => None,
    //   };
    // }

    self.inner.call(req).map(move |response| {
      #[cfg(not(feature = "stackdriver"))]
      {
        // tracing::error!(&response);
        let latency =
          (coarsetime::Clock::now_since_epoch().as_millis() - start_time) as f32 / 1000.0;
        tracing::info!("{} {} {}s", method, url, latency)
      }
      response
    })
  }
}

// struct TimingMiddleware<S> {
//   inner: S,
// }
//
// impl<S> TimingMiddleware<S> {
//   fn new(service: S) -> Self {
//     Self { inner: service }
//   }
// }
//
// impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for TimingMiddleware<S>
// where
//   S: Service<Request<ReqBody>, Response = Response<ResBody>>,
//   S::Error: Into<crate::BoxError>,
//   S::Future: Send + 'static,
//   ReqBody: Send + 'static,
//   ResBody: Send + 'static,
// {
//   type Response = S::Response;
//   type Error = S::Error;
//   type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
//
//   fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//     self.inner.poll_ready(cx)
//   }
//
//   fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
//     // let start_time = Instant::now();
//     let mut inner = self.inner.clone();
//
//     Box::pin(async move {
//       let response = inner.call(req).await?;
//       // let elapsed = start_time.elapsed();
//       info!("Request duration: {:?}", elapsed);
//       Ok(response)
//     })
//   }
// }
//
// // Middleware layer
// struct TimingLayer;
//
// impl<S> tower::Layer<S> for TimingLayer {
//   type Service = TimingMiddleware<S>;
//
//   fn layer(&self, service: S) -> Self::Service {
//     TimingMiddleware::new(service)
//   }
// }
