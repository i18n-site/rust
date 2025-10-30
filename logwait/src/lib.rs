use tokio::task::JoinSet;

pub async fn logwait<T: 'static>(mut joinset: JoinSet<Result<T, anyhow::Error>>) {
  while let Some(r) = joinset.join_next().await {
    match r {
      Err(err) => tracing::error!("join error: {}", err),
      Ok(r) => {
        if let Err(err) = r {
          tracing::error!("{}", err);
        }
      }
    }
  }
}
