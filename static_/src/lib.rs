pub use async_wrap::{OnceCell, Wrap};
pub use log;
pub use xboot::{self, init};

#[macro_export]
macro_rules! init {
  ($var:ident: $type:ident $init:expr) => {
    mod _inner {
      use $crate::OnceCell;
      pub(crate) static $var: OnceCell<super::$type> = OnceCell::const_new();
    }
    pub static $var: $crate::Wrap<$type> = $crate::Wrap(&_inner::$var);
    $crate::xboot::add!(
      {
        _inner::$var.get_or_init(|| async {
          match (|| async { $init })().await {
            Ok(r) => r,
            Err(err) => {
              $crate::log::error!("{} : {err}", stringify!($var));
              std::process::exit(1);
            }
          }
        })
      }
      .await
    );
  };
}
