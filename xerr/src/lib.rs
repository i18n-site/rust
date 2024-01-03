#[macro_export]
macro_rules! log {
  ($($r:expr),*$(,)?) => {{
    $(
      if let Err(err) = $r {
        $create::tracing::error!("{}", err);
      }
    )*
  }};
}
