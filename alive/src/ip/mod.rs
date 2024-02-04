use std::net::IpAddr;

use aok::OK;

use crate::dberr;

macro_rules! def {
  ($($mod:ident),*) => {

$(pub mod $mod;)*

pub async fn ping(kind: &str, ip: &IpAddr) -> aok::Result<()> {
  match kind {
    $(
    stringify!($mod) => {
      $mod::ping(ip).await?;
    }
    )*
    _ => dberr!(UnknownKind "{}", kind),
  }
  OK
}

  };
}

def!(ipv6_proxy);
