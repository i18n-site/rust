use crate::{
  FORBIDDEN_BYTE,
  util::{div_rem, from_bytes_be},
};

/// 将字节切片编码为 b255 格式。
///
/// 编码算法如下：
/// 1. 将输入字节视为一个大端序的大整数。
/// 2. 通过重复除以255，将该数从base256转换为base255，余数即为base255的数字。
/// 3. 为了避免在输出中使用`FORBIDDEN_BYTE`，对数字进行映射：
///    - 如果数字等于`FORBIDDEN_BYTE`的值，则将其映射为255。
///    - 其他数字保持不变。
/// 4. 输入数据中的前导零在输出的末尾表示为等量的零字节。
///
/// # 例子
///
/// ```
/// let original = b"hello";
/// let encoded = b255::encode(original);
/// assert_ne!(encoded, original);
/// ```
pub fn encode(data: impl AsRef<[u8]>) -> Vec<u8> {
  let data = data.as_ref();
  if data.is_empty() {
    return Vec::new();
  }

  // 1. 计算并分离前导零。
  // 在base转换中，前导零需要特殊处理。
  let leading_zeros = data.iter().take_while(|&&b| b == 0).count();
  let core_data = &data[leading_zeros..];

  if core_data.is_empty() {
    return vec![0; leading_zeros];
  }

  // 使用本地大数函数
  let mut num = from_bytes_be(core_data);

  // 2. 执行从 base256 到 base255 的转换。
  // 重复除以255，收集余数。
  let capacity = (core_data.len() * 2) + leading_zeros;
  let mut encoded = Vec::with_capacity(capacity);
  while num.len() > 1 || num[0] != 0 {
    let mut rem = div_rem(&mut num, 255) as u8;

    // 3. 将余数映射到目标字母表。
    // 如果余数等于被禁止的字节，则映射为255。
    if rem == FORBIDDEN_BYTE {
      rem = 255;
    }
    encoded.push(rem);
  }

  // 4. 将前导零添加到结果中。
  // 在我们的编码方案中，0 映射到 0，所以我们只需添加相应数量的零。
  encoded.extend(std::iter::repeat_n(0, leading_zeros));

  // 5. 转换过程产生的数字是小端序的。
  encoded
}
