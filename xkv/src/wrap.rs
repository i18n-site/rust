use async_lazy::Lazy;
use fred::prelude::Client;

pub struct Wrap(pub &'static Lazy<Client>);

impl std::ops::Deref for Wrap {
  type Target = Client;
  fn deref(&self) -> &Self::Target {
    self.0.get().unwrap()
  }
}
