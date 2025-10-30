pub(crate) type Digit = u64;
pub(crate) type DoubleDigit = u128;
pub(crate) const BYTES: usize = 8;
pub(crate) const BITS: usize = BYTES * 8;

pub(crate) fn from_bytes_le(bytes: &[u8]) -> Vec<Digit> {
  bytes
    .chunks(BYTES)
    .map(|c| c.iter().rev().fold(0, |a, &x| (a << 8) | x as Digit))
    .collect()
}

pub(crate) fn from_bytes_be(bytes: &[u8]) -> Vec<Digit> {
  let mut bytes = bytes.to_vec();
  bytes.reverse();
  from_bytes_le(&bytes)
}

pub(crate) fn div_rem(num: &mut Vec<Digit>, x: DoubleDigit) -> Digit {
  let mut rem = 0;
  for d in num.iter_mut().rev() {
    let a = *d as DoubleDigit | (rem << BITS);
    *d = (a / x) as Digit;
    rem = a % x;
  }
  num.truncate(num.iter().rposition(|x| *x != 0).unwrap_or(0) + 1);
  rem as Digit
}
