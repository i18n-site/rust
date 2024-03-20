pub fn b3_len(bin: impl AsRef<[u8]>) -> Box<[u8]> {
  let bin = bin.as_ref();
  let mut hasher = blake3::Hasher::new();
  hasher.update(bin);

  [
    hasher.finalize().as_bytes(),
    &(bin.len() as u64).to_le_bytes()[..],
  ]
  .concat()
  .into()
}
