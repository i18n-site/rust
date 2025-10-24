#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
mod stream;
mod traits;
pub use error::Error;
pub use stream::Stream;
pub use traits::{Conn, StreamTrait};
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(feature = "reqwest")]
pub mod util;

#[cfg(feature = "reqwest")]
mod middleware;
#[cfg(feature = "reqwest")]
pub use middleware::ProxyMiddleware;

#[cfg(all(feature = "reqwest", feature = "protocol"))]
pub(crate) mod macro_enums;

#[cfg(feature = "protocol")]
pub(crate) mod macro_conn;

macro_rules! init {
  ($($name:ident, $name_str:literal);+) => {
    $(
      #[cfg(feature = $name_str)]
      pub mod $name;
    )+

    #[cfg(feature = "protocol")]
    crate::conn!($($name, $name_str);+);

    #[cfg(all(feature = "reqwest", feature = "protocol"))]
    mod enums {
      $crate::enums!($($name,$name_str);+);
    }
  }
}

init!(shadowsocks, "shadowsocks";  hysteria2, "hysteria2");

#[cfg(all(feature = "reqwest", feature = "protocol"))]
pub use enums::{ConnFuture, Proxy, ProxyConn, StreamEnum};
