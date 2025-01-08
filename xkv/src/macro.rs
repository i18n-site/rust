#[macro_export]
macro_rules! conn {
  ($var:ident) => {
    mod __xkv {
      #[allow(non_snake_case)]
      pub mod $var {
        use $crate::{fred::prelude::Client, linkme, xboot, Lazy};

        pub static CLIENT: Lazy<Client> = Lazy::const_new(|| {
          Box::pin(async {
            let prefix = stringify!($var);
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

        fn init() -> xboot::Task {
          use std::future::IntoFuture;
          $crate::tokio::task::spawn(async {
            CLIENT.into_future().await;
            Ok(())
          })
        }

        #[linkme::distributed_slice($crate::xboot::ASYNC)]
        static INIT: xboot::AsyncFn = init;
      }
    }

    pub static $var: $crate::Wrap = $crate::Wrap(&__xkv::$var::CLIENT);
  };
}
