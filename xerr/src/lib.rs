pub use tracing;

#[macro_export]
macro_rules! log {
  ($result:expr) => {{
    match $result {
      Err(err) =>{
        $crate::tracing::error!("{}", err)
        Err(err)
      }
      Ok(r)=>{
        Ok(r)
      }
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
    $crate::log!($result).is_ok()
  }};
}
