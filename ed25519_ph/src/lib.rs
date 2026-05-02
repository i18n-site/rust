use ed25519_dalek::SigningKey;
use sha3::{Digest, Sha3_512};
use wasm_bindgen::prelude::{JsError, wasm_bindgen};

#[wasm_bindgen]
pub struct Ed25519Ph {
  hasher: Sha3_512,
  sk: SigningKey,
}

#[wasm_bindgen]
impl Ed25519Ph {
  #[wasm_bindgen(constructor)]
  pub fn new(sk: &[u8]) -> Result<Self, JsError> {
    dbg!(sk.len());
    Ok(Self {
      hasher: Sha3_512::new(),
      sk: SigningKey::from_bytes(&sk.try_into()?),
    })
  }

  #[wasm_bindgen]
  pub fn update(&mut self, data: &[u8]) {
    self.hasher.update(data);
  }

  #[wasm_bindgen]
  pub fn finish(self) -> Result<Vec<u8>, JsError> {
    Ok(
      self
        .sk
        .sign_prehashed(self.hasher, None)?
        .to_bytes()
        .to_vec(),
    )
  }
}

#[cfg(test)]
mod tests {
  use ed25519_dalek::SigningKey;

  use super::*;

  #[test]
  fn test_ed25519_ph() {
    let sk_bytes = [1u8; 32];
    let mut ph = Ed25519Ph::new(&sk_bytes).unwrap();
    ph.update(b"hello");
    ph.update(b"world");
    let sig = ph.finish().unwrap();
    assert_eq!(sig.len(), 64);

    let sk = SigningKey::from_bytes(&sk_bytes);
    let pk = sk.verifying_key();
    let mut hasher = Sha3_512::new();
    hasher.update(b"helloworld");
    let expected_sig = sk.sign_prehashed(hasher, None).unwrap();
    assert_eq!(sig, expected_sig.to_bytes().to_vec());

    pk.verify_prehashed(
      Sha3_512::new_with_prefix(b"helloworld"),
      None,
      &expected_sig,
    )
    .unwrap();
  }
}
