use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

pub fn bin_u64_li(bin: impl AsRef<[u8]>) -> Vec<u64> {
  match vbyte::decompress_list(bin.as_ref()) {
    Ok(r) => r,
    Err(_) => vec![],
  }
}

pub fn b64_decode_u64_li(bin: impl AsRef<[u8]>) -> Vec<u64> {
  let bin = bin.as_ref();
  if let Ok(r) = URL_SAFE_NO_PAD.decode(bin) {
    return bin_u64_li(r);
  }
  vec![]
}

// TODO encode
// #[wasm_bindgen]
// pub fn vbyteD(vs: &[u8]) -> Result<Vec<f64>> {
//   match vbyte::decompress_list(vs) {
//     Ok(r) => Ok(r.into_iter().map(|i| i as f64).collect()),
//     Err(err) => Err(wasm_bindgen::JsError::new(err)),
//   }
// }
//
// #[wasm_bindgen]
// pub fn vbyteE(vs: &[f64]) -> Vec<u8> {
//   vbyte::compress_list(&vs.iter().map(|i| *i as u64).collect::<Vec<_>>())
// }
// #[wasm_bindgen]
// pub fn b64VbyteE(vs: &[f64]) -> String {
//   URL_SAFE_NO_PAD.encode(vbyteE(vs))
// }
