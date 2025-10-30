mod error;
pub use error::DecodeError;

#[cfg(feature = "decode")]
pub mod decode;
#[cfg(feature = "decode")]
pub use decode::decode;

#[cfg(feature = "encode")]
pub mod encode;
#[cfg(feature = "encode")]
pub use encode::encode;
#[cfg(any(feature = "decode", feature = "encode"))]
mod util;

pub const FORBIDDEN_BYTE: u8 = b':';
