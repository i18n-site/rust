#![feature(trait_alias)]

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
