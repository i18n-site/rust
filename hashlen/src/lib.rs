#![cfg_attr(docsrs, feature(doc_cfg))]

pub fn hashlen(input: impl AsRef<[u8]>) -> Vec<u8> {
  let input = input.as_ref();
  if input.len() < 17 {
    return input.to_vec();
  }

  [
    &gxhash::gxhash128(input, 0).to_le_bytes()[..],
    &intbin::to_bin(input.len())[..],
  ]
  .concat()
}
