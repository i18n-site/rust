use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use intbin::{bin_u64, to_bin};

pub fn u64_b64(n: u64) -> String {
  URL_SAFE_NO_PAD.encode(to_bin(n))
}

pub fn b64_u64(bin: impl AsRef<[u8]>) -> u64 {
  if let Ok(r) = URL_SAFE_NO_PAD.decode(bin.as_ref()) {
    return bin_u64(r);
  }
  0
}
