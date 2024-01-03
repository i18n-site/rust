pub use tracing;

#[macro_export]
macro_rules! log {
  ($result:expr) => {{
    if let Err(err) = $result {
      $crate::tracing::error!("{}", err);
    }
  }};
  ($($result:expr),+$(,)?) => {{
    $(
      $crate::log!($result);
    )+
  }}
}
