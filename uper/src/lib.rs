#![cfg_attr(docsrs, feature(doc_cfg))]

pub use const_str;

mod dns_check;
mod uper;

pub use clap::{ArgAction, ArgMatches, Command, arg};
pub use dns_check::dns_check;
pub use uper::Uper;

#[cfg(feature = "conf")]
mod conf;
#[cfg(feature = "conf")]
pub use conf::load;

#[cfg(feature = "conf")]
#[macro_export]
macro_rules! load {
  ($host_li: expr, $pk: expr, $cmd_build: expr, $run: expr) => {{
    $crate::load(
      $host_li,
      $pk,
      $cmd_build,
      $run,
      env!("CARGO_PKG_NAME"),
      [
        $crate::const_str::parse!(env!("CARGO_PKG_VERSION_MAJOR"), u64),
        $crate::const_str::parse!(env!("CARGO_PKG_VERSION_MINOR"), u64),
        $crate::const_str::parse!(env!("CARGO_PKG_VERSION_PATCH"), u64),
      ],
    )
    .await
  }};
}
