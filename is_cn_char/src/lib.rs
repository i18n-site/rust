/// 判断一个字符是否为中文字符。
///
/// 这包括了 CJK 统一表意文字的主要范围、扩展区、部首、笔画和兼容字符等。
pub fn is_cn_char(i: char) -> bool {
  let i = i as u32;
  for [b, e] in [
    [0x4E00, 0x9FA5],   // CJK 统一表意文字 (基本区)
    [0x9FA6, 0x9FCB],   // CJK 统一表意文字 (新增)
    [0x3400, 0x4DB5],   // CJK 统一表意文字扩展 A
    [0x20000, 0x2A6D6], // CJK 统一表意文字扩展 B
    [0x2A700, 0x2B734], // CJK 统一表意文字扩展 C
    [0x2B740, 0x2B81D], // CJK 统一表意文字扩展 D
    [0x2F00, 0x2FD5],   // 康熙部首
    [0x2E80, 0x2EF3],   // CJK 部首补充
    [0xF900, 0xFAD9],   // CJK 兼容表意文字
    [0x2F800, 0x2FA1D], // CJK 兼容表意文字补充
    [0xE815, 0xE86F],   // 私人使用区 (一些自定义字符)
    [0xE400, 0xE5E8],   // 私人使用区
    [0xE600, 0xE6CF],   // 私人使用区
    [0x31C0, 0x31E3],   // CJK 笔画
    [0x2FF0, 0x2FFB],   // 表意文字描述字符
    [0x3105, 0x3120],   // 注音符号
    [0x31A0, 0x31BA],   // 注音符号扩展
  ] {
    if i >= b && i <= e {
      return true;
    }
  }
  if i == 0x3007 {
    // 〇 (零)
    return true;
  }
  false
}

///// 判断一个字符是否为日文字符。
/////
///// 这包括平假名、片假名、常见的 CJK 汉字以及日文标点和符号。
//pub fn is_ja_char(c: char) -> bool {
//  let c = c as u32;
//  for &[start, end] in &[
//    [0x3040, 0x309F], // 平假名 (Hiragana)
//    [0x30A0, 0x30FF], // 片假名 (Katakana)
//    [0x4E00, 0x9FAF], // CJK 统一表意文字 (常用汉字 Kanji)
//    [0x3400, 0x4DBF], // CJK 统一表意文字扩展 A
//    [0xFF00, 0xFFEF], // 全角罗马字符和半角片假名
//    [0x3000, 0x303F], // 日文标点符号
//    [0x31F0, 0x31FF], // 片假名语音扩展
//  ] {
//    if c >= start && c <= end {
//      return true;
//    }
//  }
//  false
//}
//
///// 判断一个字符是否为泰文字符。
//pub fn is_th_char(c: char) -> bool {
//  let c = c as u32;
//  // 泰文 Unicode 范围
//  (0x0E00..=0x0E7F).contains(&c)
//}
//
///// 判断一个字符是否为繁体中文字符。
/////
///// Unicode 中没有严格区分简体和繁体，因此此函数覆盖了所有 CJK 表意文字的主要范围。
///// 它与 `is_cn_char` 的范围非常相似，但更侧重于表意文字本身。
//pub fn is_zhtw_char(c: char) -> bool {
//  let i = c as u32;
//  for &[b, e] in &[
//    [0x4E00, 0x9FFF],   // CJK 统一表意文字
//    [0x3400, 0x4DBF],   // CJK 统一表意文字扩展 A
//    [0x20000, 0x2A6DF], // CJK 统一表意文字扩展 B
//    [0x2A700, 0x2B73F], // CJK 统一表意文字扩展 C
//    [0x2B740, 0x2B81F], // CJK 统一表意文字扩展 D
//    [0x2B820, 0x2CEAF], // CJK 统一表意文字扩展 E
//    [0x2CEB0, 0x2EBEF], // CJK 统一表意文字扩展 F
//    [0xF900, 0xFAFF],   // CJK 兼容表意文字
//    [0x2F800, 0x2FA1F], // CJK 兼容表意文字补充
//  ] {
//    if i >= b && i <= e {
//      return true;
//    }
//  }
//  false
//}
//
///// 判断一个字符是否为高棉语（柬埔寨语）字符。
//pub fn is_km_char(c: char) -> bool {
//  let c = c as u32;
//  // 高棉语 Unicode 范围
//  (0x1780..=0x17FF).contains(&c)
//}
//
///// 判断一个字符是否为老挝语（寮语）字符。
//pub fn is_lo_char(c: char) -> bool {
//  let c = c as u32;
//  // 老挝语 Unicode 范围
//  (0x0E80..=0x0EFF).contains(&c)
//}
