#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]

pub fn hashlen(input: impl AsRef<[u8]>) -> Vec<u8> {
  let input = input.as_ref();
  [
    &gxhash::gxhash128(input, 0).to_le_bytes()[..],
    &intbin::to_bin(input.len())[..],
  ]
  .concat()
}
