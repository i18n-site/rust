use num_traits::PrimInt;

pub fn e<T: PrimInt>(mut value: T) -> String {
  if value == T::zero() {
    return "0".to_string();
  }

  let radix = T::from(36).unwrap();
  let mut result = String::new();

  while value > T::zero() {
    let remainder = value % radix;
    let digit = remainder.to_u8().unwrap();

    let c = match digit {
      0..=9 => (b'0' + digit) as char,
      10..=35 => (b'A' + (digit - 10)) as char,
      _ => unreachable!(),
    };

    result.push(c);
    value = value / radix;
  }

  // 反转字符串，因为我们是从最低位开始构建的
  result.chars().rev().collect()
}
