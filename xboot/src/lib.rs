use aok::Result;
pub use gensym::gensym;
pub use linkme::distributed_slice;
pub use paste::paste;
pub use tokio;

pub type Task = tokio::task::JoinHandle<Result<()>>;

pub type AsyncFn = fn() -> Task;

#[distributed_slice]
pub static ASYNC: [AsyncFn];

pub async fn init() -> Result<()> {
  for i in ASYNC {
    i().await??;
  }
  Ok(())
}

#[macro_export]
macro_rules! add {
  ($init:expr) => {
    $crate::gensym! {$crate::add! {$init}}
  };
  ($id:expr, $init:expr) => {
    $crate::paste! {
    fn [<xboot_init_ $id>]() -> $crate::Task {
      $crate::tokio::task::spawn(async {
        $init;
        Ok(())
      })
    }
    #[$crate::distributed_slice($crate::ASYNC)]
    static [<ASYNC_INIT_ $id>]: $crate::AsyncFn = [<xboot_init_ $id>];
    }
  };
}
