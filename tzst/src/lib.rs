#![feature(type_alias_impl_trait)]

#[cfg(feature = "w")]
pub mod w;

#[cfg(feature = "w")]
pub use w::W;

#[cfg(feature = "r")]
pub mod r;

#[cfg(feature = "r")]
pub use r::r;

#[cfg(feature = "zst")]
pub mod zst;
