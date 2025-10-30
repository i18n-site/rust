use aok::{OK, Result};
use b255::{DecodeError, FORBIDDEN_BYTE, decode, encode};

#[static_init::constructor(0)]
extern "C" fn _log_init() {
  log_init::init();
}

#[test]
fn test_empty() -> Result<()> {
  assert_eq!(encode(&[]), &[]);
  assert_eq!(decode(&[])?, &[]);
  OK
}

#[test]
fn test_simple() -> Result<()> {
  let original = ":-:您好:".as_bytes();
  let encoded = encode(original);
  assert!(!encoded.contains(&FORBIDDEN_BYTE));
  let decoded = decode(&encoded)?;
  assert_eq!(decoded, original);
  OK
}

#[test]
fn test_all_bytes() -> Result<()> {
  let original: Vec<u8> = (0..=u8::MAX).collect();
  let encoded = encode(&original);
  assert!(!encoded.contains(&FORBIDDEN_BYTE));
  let decoded = decode(&encoded)?;
  assert_eq!(decoded, &original[..]);
  OK
}

#[test]
fn test_zeros() -> Result<()> {
  for original in [[0, 0, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]] {
    let encoded = encode(&original);
    assert!(!encoded.contains(&FORBIDDEN_BYTE));
    let decoded = decode(&encoded)?;
    assert_eq!(decoded, original);
  }
  OK
}

#[test]
fn test_decode_error() -> Result<()> {
  let invalid_input = &[0, 1, FORBIDDEN_BYTE, 2]; // 包含禁止的字节
  let result = decode(invalid_input);
  assert!(matches!(
    result,
    Err(DecodeError::InvalidByte(FORBIDDEN_BYTE))
  ));
  OK
}
