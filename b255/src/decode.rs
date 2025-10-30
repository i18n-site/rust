use crate::{
  DecodeError, FORBIDDEN_BYTE,
  util::{BITS, BYTES, Digit, DoubleDigit},
};

pub(crate) fn to_bytes_le(num: &[Digit]) -> Vec<u8> {
  let mut bytes: Vec<_> = num
    .iter()
    .flat_map(|c| (0..BYTES).map(move |i| (c >> (i * 8)) as u8))
    .collect();
  bytes.truncate(bytes.iter().rposition(|x| *x != 0).unwrap_or(0) + 1);
  bytes
}

pub(crate) fn to_bytes_be(num: &[Digit]) -> Vec<u8> {
  let mut bytes = to_bytes_le(num);
  bytes.reverse();
  bytes
}

pub(crate) fn add(num: &mut Vec<Digit>, x: DoubleDigit) {
  let mut carry = x;
  for d in num.iter_mut() {
    carry += *d as DoubleDigit;
    *d = carry as Digit;
    carry >>= BITS;
    if carry == 0 {
      break;
    }
  }
  if carry != 0 {
    num.push(carry as Digit)
  }
}

pub(crate) fn mul(num: &mut Vec<Digit>, x: DoubleDigit) {
  let mut carry = 0;
  for d in num.iter_mut() {
    carry += *d as DoubleDigit * x;
    *d = carry as Digit;
    carry >>= BITS;
  }
  if carry != 0 {
    num.push(carry as Digit)
  }
}
/// 将 b255 编码的字节切片解码回原始字节。
///
/// 解码算法是编码的逆过程：
/// 1. 将末尾的零（代表原始数据中的前导零）分离出来。
/// 2. 对核心数据中的每个字节，应用逆映射以获得base255的数字：
///    - 字节255被映射回`FORBIDDEN_BYTE`的值。
///    - 其他字节值保持不变。
/// 3. 使用霍纳法则，将base255的数字转换回base256的大整数。
/// 4. 将大整数转换回字节序列。
/// 5. 将前导零和解码后的主体组合起来得到最终结果。
///
/// 如果输入包含被禁止的字节 `FORBIDDEN_BYTE`，则返回 `DecodeError::InvalidByte`。
///
/// # 例子
///
/// ```
/// let encoded = b255::encode(b"hello");
/// let decoded = b255::decode(&encoded).unwrap();
/// assert_eq!(decoded, b"hello");
/// ```
pub fn decode(data: impl AsRef<[u8]>) -> Result<Vec<u8>, DecodeError> {
  let data = data.as_ref();
  if data.is_empty() {
    return Ok(Vec::new());
  }

  // 1. 计算并分离末尾的零（代表原始数据中的前导零）。
  let trailing_zeros = data.iter().rev().take_while(|&&b| b == 0).count();
  let core_data = &data[..data.len() - trailing_zeros];

  if core_data.is_empty() {
    return Ok(vec![0; trailing_zeros]);
  }

  let mut num: Vec<Digit> = vec![0];

  // 2. 从 base255 转换到 base256。
  // 输入是小端序 [d0, d1, ..., dn]，我们需要从 dn 开始处理。
  for &byte in core_data.iter().rev() {
    // 逆向映射字节到数字
    let digit = match byte {
      FORBIDDEN_BYTE => return Err(DecodeError::InvalidByte(FORBIDDEN_BYTE)),
      255 => FORBIDDEN_BYTE,
      b => b,
    };
    // 使用霍纳法则进行基数转换。
    mul(&mut num, 255);
    add(&mut num, digit.into());
  }

  // 3. 将大数转换回字节向量。
  let decoded_bytes = to_bytes_be(&num);

  // 4. 组合前导零和解码后的主体。
  let mut result = vec![0; trailing_zeros];
  result.extend_from_slice(&decoded_bytes);

  Ok(result)
}
