use volo_grpc::request::Request;

#[derive(Clone)]
pub struct LogService<S>(S);

#[volo::service]
impl<Cx, S, T> volo::Service<Cx, Request<T>> for LogService<S>
where
  S: Send + 'static + volo::Service<Cx, Request<T>> + Sync,
  Cx: Send + 'static,
  T: Send,
{
  async fn call(&self, cx: &mut Cx, req: Request<T>) -> Result<S::Response, S::Error> {
    let meta = req.metadata();
    let rip = meta
      .get("rip")
      .map(|i| String::from_utf8_lossy(i.as_bytes()))
      .unwrap_or_default()
      .to_string();

    let user_agent = meta
      .get("user-agent")
      .map(|i| String::from_utf8_lossy(i.as_bytes()))
      .unwrap_or_default()
      .to_string();

    let now = coarsetime::Instant::now();
    let resp = self.0.call(cx, req).await;
    tracing::info!("{rip} {user_agent} {}ms", now.elapsed().as_millis());
    resp
  }
}

pub struct Log;

impl<S> volo::Layer<S> for Log {
  type Service = LogService<S>;

  fn layer(self, inner: S) -> Self::Service {
    LogService(inner)
  }
}
