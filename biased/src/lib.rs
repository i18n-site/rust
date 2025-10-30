#![cfg_attr(docsrs, feature(doc_cfg))]

// 导入必要的 trait 和模块。
// Import necessary traits and modules.
use std::{
  cmp::Ord,
  ops::{Bound, RangeBounds},
};

use num_traits::{FromPrimitive, Num, ToPrimitive};
use rand::random;

/// # Biased Random Number Generation
///
/// Generates a non-uniformly distributed random integer in a given range.
///
/// The function accepts any type that implements `RangeBounds<T>`, such as `a..b` or `a..=b`.
/// Note: Ranges with an unbounded end (e.g., `a..` or `..`) are not supported. For ranges with an unbounded start (e.g., `..b`), the start is assumed to be 0.
///
/// The distribution is controlled by a bias parameter, which can make smaller or larger integers more likely.
///
/// # Arguments
///
/// * `range` - The range of random numbers to generate, implementing `RangeBounds<T>`.
/// * `bias` - The bias strength parameter. Must be a positive number.
///   * `bias > 1`: Smaller numbers are more likely. The larger the bias, the more skewed the result is towards the start of the range.
///   * `bias = 1`: Approaches a standard uniform distribution.
///   * `0 < bias < 1`: Larger numbers are more likely (skewed towards the end of the range).
///
/// # Returns
///
/// Returns a random integer within the `range`.
/// If the range is too large or invalid, it returns the start of the range as a fallback.
///
/// # Type Parameters
///
/// * `T` - An integer type that implements `Num`, `Ord`, `ToPrimitive`, `FromPrimitive`, and `Copy`.
///
/// # Examples
///
/// ```
/// // Generate a random number in the range [0, 100), biased towards smaller values.
/// let num = biased::rng(0..100, 3.0);
/// assert!(num >= 0 && num < 100);
///
/// // Use an inclusive range [0, 100]
/// let num_inclusive = biased::rng(0..=100, 3.0);
/// assert!(num_inclusive >= 0 && num_inclusive <= 100);
/// ```
///
/// ---
///
/// # 偏向性随机数生成
///
/// 在指定范围内生成一个非均匀分布的随机整数。
///
/// 函数接受任何实现了 `RangeBounds<T>` 的类型，例如 `a..b` 或 `a..=b`。
/// 注意：不支持末端无限的范围 (例如 `a..` 或 `..`)。对于起始无限的范围 (例如 `..b`)，起始值默认为0。
///
/// 分布由一个偏向参数 (bias) 控制，可以使较小或较大的整数更有可能被生成。
///
/// # 参数
///
/// * `range` - 要生成的随机数范围，实现了 `RangeBounds<T>`。
/// * `bias` - 偏向强度参数，必须是正数。
///   * `bias > 1`: 数值越小概率越高，结果越偏向范围的起始值。
///   * `bias = 1`: 接近标准均匀分布。
///   * `0 < bias < 1`: 数值越大概率越高，结果越偏向范围的结束值。
///
/// # 返回值
///
/// 返回一个在 `range` 内的随机整数。
///
/// 如果范围过大或无效，则返回范围的起始值作为后备。
///
/// # 类型参数
///
/// * `T` - 一个整数类型，需要实现 `Num`、`Ord`、`ToPrimitive`、`FromPrimitive` 和 `Copy` trait。
///
/// # 示例
///
/// ```
/// // 生成一个在 [0, 100) 范围内的随机数，偏向于较小的值。
/// let num = biased::rng(0..100, 3.0);
/// assert!(num >= 0 && num < 100);
///
/// // 使用包含性范围 [0, 100]
/// let num_inclusive = biased::rng(0..=100, 3.0);
/// assert!(num_inclusive >= 0 && num_inclusive <= 100);
/// ```
pub fn rng<T, R>(range: R, bias: f64) -> T
where
  T: Num + Ord + ToPrimitive + FromPrimitive + Copy,
  R: RangeBounds<T>,
{
  let start = match range.start_bound() {
    Bound::Included(s) => *s,
    Bound::Excluded(s) => *s + T::one(),
    Bound::Unbounded => T::zero(),
  };

  let end = match range.end_bound() {
    Bound::Included(e) => *e + T::one(),
    Bound::Excluded(e) => *e,
    Bound::Unbounded => return start, // 不支持末端无限的范围 (Unsupported unbounded end)
  };

  if start >= end {
    return start;
  }

  let len = end - start;

  // 将范围长度转换为 f64。如果数值过大无法表示，则返回范围的起始值。
  // Convert the range length to f64. If it's too large to be represented, return the start of the range.
  let len_f64 = match len.to_f64() {
    Some(val) => val,
    None => return start,
  };

  // 1. 获取一个 [0, 1) 之间的标准均匀分布随机数。
  // 1. Get a standard uniform random number in [0, 1).
  let rng: f64 = random();

  // 2. 使用幂函数来“扭曲”这个分布。
  // 2. "Warp" the distribution using a power function.
  let biased_rng = rng.powf(bias);

  // 3. 将偏向随机数映射到 [0, len) 的整数偏移量范围。
  // 3. Map the biased random number to the integer offset range [0, len).
  let offset_f64 = biased_rng * len_f64;

  // 将结果转换回整数类型 T。
  // 此转换不应失败，因为可以保证结果在有效范围内。
  // Convert the result back to the integer type T.
  // This conversion should not fail as the result is guaranteed to be in the valid range.
  let offset = match T::from_f64(offset_f64) {
    Some(value) => value,
    // 如果逻辑正确，这个情况是不可达的。
    // 返回 0 作为一个安全的后备值，以避免 panic。
    // This case should be unreachable if the logic is correct.
    // Return zero as a safe fallback to avoid panicking.
    None => T::zero(),
  };

  start + offset
}
