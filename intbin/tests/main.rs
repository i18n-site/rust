use aok::{OK, Void};
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[test]
fn test() -> Void {
  // u64 test
  let n: u64 = 123456;
  let bin = intbin::to_bin(n);
  info!("{n} -> {:?}", &bin);
  assert_eq!(n, intbin::bin_u64(&bin));

  // u16 test
  let n16: u16 = 12345;
  let bin16 = intbin::to_bin(n16);
  assert_eq!(n16, intbin::bin_u16(&bin16));

  // u8 test
  let n8: u8 = 123;
  let bin8 = intbin::u8_bin(n8);
  assert_eq!(n8, intbin::bin_u8(&bin8));

  // 0 test (empty)
  let n_zero: u64 = 0;
  let bin_zero = intbin::to_bin(n_zero);
  assert!(bin_zero.is_empty());
  assert_eq!(0u64, intbin::bin_u64(&bin_zero));
  assert_eq!(0u16, intbin::bin_u16(&bin_zero));
  assert_eq!(0u8, intbin::bin_u8(&bin_zero));

  // Overflow slice size test (safety test for min(N))
  let large_bin = vec![1u8, 2u8, 3u8, 4u8, 5u8];
  // bin_u16 should only read first 2 bytes: [1, 2] -> 0x0201 = 513
  assert_eq!(513u16, intbin::bin_u16(&large_bin));
  // bin_u8 should only read first 1 byte: 1
  assert_eq!(1u8, intbin::bin_u8(&large_bin));

  OK
}
