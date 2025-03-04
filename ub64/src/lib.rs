pub use base64::{engine::general_purpose::URL_SAFE_NO_PAD, DecodeError, Engine};

pub fn b64e(bin: impl AsRef<[u8]>) -> String {
  URL_SAFE_NO_PAD.encode(bin)
}

pub fn b64d(bin: impl AsRef<[u8]>) -> Result<Vec<u8>, DecodeError> {
  URL_SAFE_NO_PAD.decode(bin)
}

#[cfg(feature = "u64")]
mod u64;

#[cfg(feature = "u64")]
pub use u64::{b64_u64, u64_b64};

#[cfg(feature = "u64li")]
mod u64li;

#[cfg(feature = "u64li")]
pub use u64li::{b64_decode_u64_li, bin_u64_li};
