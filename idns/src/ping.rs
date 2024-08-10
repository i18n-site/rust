use std::net::SocketAddr;

use enum_dispatch::enum_dispatch;
use futures::stream::{FuturesUnordered, StreamExt};
use tokio::{
  net::TcpStream,
  time::{sleep, Duration},
};

#[derive(Debug)]
pub enum PingResult {
  Timeout,
  Addr(SocketAddr),
  Err,
}

macro_rules! addr {
  ($($ip:expr),+) => {
  SocketAddr::new(
    crate::ip!($($ip),+),
    443,
  )
  };
}

#[enum_dispatch]
trait PingCtrl {
  async fn ping(self) -> PingResult;
}

#[enum_dispatch(PingCtrl)]
pub enum Pinger {
  Ping,
  Timeout,
}

pub struct Ping(SocketAddr);

impl PingCtrl for Ping {
  async fn ping(self) -> PingResult {
    if TcpStream::connect(&self.0).await.is_ok() {
      PingResult::Addr(self.0)
    } else {
      PingResult::Err
    }
  }
}

pub struct Timeout(u64);

impl PingCtrl for Timeout {
  async fn ping(self) -> PingResult {
    sleep(Duration::from_secs(self.0)).await;
    PingResult::Timeout
  }
}

pub async fn use_ipv6() -> bool {
  let addr_li: &[SocketAddr] = &[
    addr!(0x2606, 0x4700, 0x4700, 0, 0, 0, 0, 0x64),
    addr!(0x2400, 0x3200, 0, 0, 0, 0, 0, 0x0001),
    addr!(0x2400, 0xda00, 0, 0, 0, 0, 0, 0x6666),
    addr!(0x2001, 0x4860, 0x4860, 0, 0, 0, 0, 0x8888),
    addr!(120, 53, 53, 53),
    addr!(1, 1, 1, 1),
    addr!(8, 8, 8, 8),
    addr!(223, 5, 5, 5),
  ];

  let mut timeout = 3;

  'out: loop {
    let mut ing = FuturesUnordered::new();

    for addr in addr_li {
      ing.push(Pinger::from(Ping(*addr)).ping());
    }
    ing.push(Pinger::from(Timeout(timeout)).ping());
    if timeout < 10 {
      timeout += 1;
    }

    while let Some(result) = ing.next().await {
      match result {
        PingResult::Addr(addr) => return addr.is_ipv6(),
        PingResult::Timeout => {
          continue 'out;
        }
        _ => {}
      }
    }
  }
}
