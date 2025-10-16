pub use anyhow;
pub use log as tracing;

#[macro_export]
macro_rules! ignore {
  ($expr:expr) => {{
    let r = (async move || Ok::<_, $crate::anyhow::Error>($expr))();
    if let Err(err) = r.await {
      $crate::tracing::error!("{}", err);
    }
  }};
}

#[macro_export]
macro_rules! ok_or {
  ($expr:expr,$default:expr) => {{
    let result = (move || Ok::<_, $crate::anyhow::Error>($expr))();
    match result {
      Ok(r) => r,
      Err(err) => {
        $crate::tracing::error!("{}", err);
        $default
      }
    }
  }};
}

#[macro_export]
macro_rules! log {
  ($result:expr) => {{
    if let Err(err) = $result {
      $crate::tracing::error!("{}", err);
    }
  }};
  ($($result:expr),+$(,)?) => {{
    $(
      $crate::tracing!($result);
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
