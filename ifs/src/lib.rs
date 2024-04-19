#![feature(trait_alias)]

#[cfg(feature = "is_hidden")]
mod is_hidden;

#[cfg(feature = "is_hidden")]
pub use is_hidden::is_hidden;

#[cfg(feature = "fs")]
mod fs;

#[cfg(feature = "fs")]
pub use fs::*;

#[cfg(feature = "hash")]
mod hash;

#[cfg(feature = "hash")]
pub use hash::*;

#[cfg(feature = "txz_hash_d")]
pub mod txz_hash_d;

#[cfg(feature = "dir")]
pub mod dir;

#[cfg(feature = "conf")]
pub mod conf;

#[cfg(feature = "rsync")]
mod rsync;

#[cfg(feature = "rsync")]
pub use rsync::rsync;

#[cfg(feature = "b3_len")]
mod b3_len;

#[cfg(feature = "b3_len")]
pub use b3_len::b3_len;

#[cfg(feature = "confdir")]
mod confdir;

#[cfg(feature = "confdir")]
pub use confdir::confdir;
