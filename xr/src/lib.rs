use linkme::distributed_slice;
pub use xkv::fred;

// #[macro_export]
// macro_rules! conn {
//   ($var:ident = $prefix:ident) => {
pub mod __xkv {
  use async_lazy::Lazy;
  use xkv::Client;

  pub static R: Lazy<Client> = Lazy::const_new(|| {
    Box::pin(async {
      let prefix = "R";
      let mut retry = 0;
      loop {
        match xkv::conn(prefix).await {
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

  // pub static R: &xkv::Client = unsafe {};

  // pub static $var: $crate::Lazy<$crate::Client> = $crate::Lazy::const_new(|| {
  //   Box::pin(async move {
  //   })
  // });
}

fn init() -> xboot::Task {
  use std::future::IntoFuture;
  tokio::task::spawn(async {
    __xkv::R.into_future().await;
    Ok(())
  })
}

#[distributed_slice(xboot::ASYNC)]
static INIT: xboot::AsyncFn = init;

pub static R: xkv::Wrap = xkv::Wrap(&__xkv::R);
