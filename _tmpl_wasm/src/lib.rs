// use js_sys::{Array, Uint8Array};
// use ed25519_dalek::{Signature, SigningKey, VerifyingKey, ed25519::signature::Verifier};
// use wasm_bindgen::prelude::{JsError, wasm_bindgen};
// use sha3::{Digest, Sha3_512};
//
// #[wasm_bindgen]
// pub struct Ed25519Ph {
//   hasher: Sha3_512,
//   sk: SigningKey,
// }
//
// #[wasm_bindgen]
// impl Ed25519Ph {
//   #[wasm_bindgen(constructor)]
//   pub fn new(sk: &[u8]) -> Result<Self, JsError> {
//     dbg!(sk.len());
//     Ok(Self {
//       hasher: Sha3_512::new(),
//       sk: SigningKey::from_bytes(&sk.try_into()?),
//     })
//   }
//
//   #[wasm_bindgen]
//   pub fn update(&mut self, data: &[u8]) {
//     self.hasher.update(data);
//   }
//
//   #[wasm_bindgen]
//   pub fn finish(self) -> Result<Vec<u8>, JsError> {
//     Ok(
//       self
//         .sk
//         .sign_prehashed(self.hasher, None)?
//         .to_bytes()
//         .to_vec(),
//     )
//   }
// }
