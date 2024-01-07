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

#[macro_export]
macro_rules! is_ok {
  ($result:expr) => {{
    match $result {
      Ok(_) => true,
      Err(err) => {
        $crate::tracing::error!("{}", err);
        false
      }
    }
  }};
}

#[macro_export]
macro_rules! ok {
  ($result:expr) => {{
    match $result {
      Ok(r) => Ok(r),
      Err(err) => {
        $crate::tracing::error!("{}", err);
        Err(err)
      }
    }
  }};
}
