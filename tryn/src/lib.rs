pub async fn tryn<F, T, E, Fut>(n: usize, mut f: F) -> aok::Result<T>
where
  Fut: std::future::Future<Output = Result<T, E>> + Send,
  F: FnMut() -> Fut,
  T: Send + 'static,
  E: Send + Into<aok::Error> + 'static,
{
  let mut attempt = 0;
  loop {
    match f().await {
      Ok(result) => return Ok(result),
      Err(e) => {
        let e = e.into();
        attempt += 1;
        if attempt >= n {
          return Err(e);
        }
        tracing::error!("{} {}", attempt, e);
      }
    }
  }
}

pub async fn retry<F, T, E, Fut>(f: F) -> aok::Result<T>
where
  Fut: std::future::Future<Output = Result<T, E>> + Send,
  F: FnMut() -> Fut,
  T: Send + 'static,
  E: Send + Into<aok::Error> + 'static,
{
  tryn(2, f).await
}
