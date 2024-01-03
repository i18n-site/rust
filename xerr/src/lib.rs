#[macro_export]
macro_rules! log {
  ($result:expr) => {{
    if let Err(err) = $result {
      tracing::error!("{}", err);
    }
  }};
}
