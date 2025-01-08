#[macro_export]
macro_rules! conn {
  ($var:ident ) => {
    use linkme::distributed_slice;
    pub mod __xkv {
      use $crate::fred::{Client, Lazy};

      pub static $var: Lazy<Client> = Lazy::const_new(|| {
        let prefix = stringify!($var)
        Box::pin(async {
          let mut retry = 0;
          loop {
            match $crate::conn(prefix).await {
              Ok(r) => return r,
              Err(err) => {
                eprintln!("❌ Connection Redis {prefix} : {}", err);
                if retry > 99 {
                  std::process::exit(1);
                }
                retry += 1;
              }
            }
          }
        })
      });
    }
    fn init() -> xboot::Task {
      use std::future::IntoFuture;
      tokio::task::spawn(async {
        __xkv::$var.into_future().await;
        Ok(())
      })
    }

    #[distributed_slice(xboot::ASYNC)]
    static INIT: xboot::AsyncFn = init;

    pub static $var: $crate::Wrap = $crate::Wrap(&__$crate::$var);
  };
}
