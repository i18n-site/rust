use std::ops::Deref;

pub use tokio::sync::OnceCell;

pub struct Wrap<T: 'static>(pub &'static OnceCell<T>);

impl<T: 'static> Deref for Wrap<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    self.0.get().unwrap()
  }
}
