#![feature(trait_alias)]
use std::{future::Future, pin::Pin};

use tokio_stream::Stream;
pub use tokio_stream::StreamExt;
pub trait Rv = Send + 'static;
pub trait Rt<R> = Future<Output = R> + Send + 'static;
pub trait Func<T, F> = Fn(T) -> F + Send + 'static;

pub trait MapAwait<T, IntoIter: IntoIterator<Item = T>> {
  fn map_unordered<R: Rv, RtR: Rt<R>>(
    self,
    并发: usize,
    func: impl Func<T, RtR>,
  ) -> Pin<Box<dyn Stream<Item = R> + Send>>
  where
    <IntoIter as IntoIterator>::IntoIter: Send + 'static;
}

impl<T, IntoIter: IntoIterator<Item = T>> MapAwait<T, IntoIter> for IntoIter {
  fn map_unordered<R: Rv, RtR: Rt<R>>(
    self,
    并发: usize,
    func: impl Func<T, RtR>,
  ) -> Pin<Box<dyn Stream<Item = R> + Send>>
  where
    <IntoIter as IntoIterator>::IntoIter: Send + 'static,
  {
    macro_rules! send {
      ($send:expr, $t:expr) => {
        let send = $send.clone();
        let ing = func($t);
        tokio::spawn(async move { send.send(ing.await).await });
      };
    }

    let (send, recv) = kanal::bounded_async(0);
    let mut iter = self.into_iter();

    let mut n = 0;

    while let Some(t) = iter.by_ref().next() {
      send!(send, t);
      n += 1;
      if n == 并发 {
        break;
      }
    }

    let mut some_send = if n == 0 { None } else { Some(send) };
    Box::pin(async_stream::stream! {
      while let Ok(item) = recv.recv().await {
        if let Some(ref send) = some_send {
          if let Some(t) = iter.next() {
            send!(send,t);
          }else{
            some_send = None;
          }
        };
        yield item;
      }
    })
  }
}
