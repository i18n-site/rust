use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use intbin::{bin_u64, u64_bin};

pub fn u64_b64(n: u64) -> String {
  URL_SAFE_NO_PAD.encode(&u64_bin(n))
}

pub fn b64_u64(bin: impl AsRef<[u8]>) -> u64 {
  if let Ok(r) = URL_SAFE_NO_PAD.decode(bin.as_ref()) {
    return bin_u64(r);
  }
  0
}
