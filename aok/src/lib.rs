pub use anyhow::*;

pub const OK: std::result::Result<(), Error> = std::result::Result::Ok(());

pub type Null = Result<()>;

#[macro_export]
macro_rules! err {
  ($($tt:tt)*) => {
    Err($crate::anyhow!($($tt)*))
  };
}

#[macro_export]
macro_rules! throw {
  ($msg:literal $(,)?) => {
    return Err($crate::anyhow!($msg));
  };
  ($err:expr $(,)?) => {
    return Err($crate::anyhow!($err));
  };
  ($fmt:expr, $($arg:tt)*) => {
    return Err($crate::anyhow!($fmt, $($arg)*));
  };
  ($err:expr ; $($arg:tt)*) => {{
    match $err {
      Err(err)=>{
        let mut msg = format!($($arg)*);
        msg.push_str(&format!(" : {}", err));
        return Err($crate::Error::msg(msg));
      }
      Ok(r) => r
    }
  }};
}
