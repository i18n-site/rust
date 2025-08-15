use aok::{OK, Void};
use is_cn_char::{is_cn_char, is_ja_char, is_km_char, is_lo_char, is_th_char, is_zhtw_char};
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[test]
fn main() {
  let test_chars = [
    '你', '好', // 中文
    'こ', 'ん', // 日语 (平假名)
    'カ', // 日语 (片假名)
    '語', // 日语 (汉字)
    'ก', 'ข', // 泰语
    '龍', '門', // 繁体中文
    'ស', 'ដ', // 高棉语
    'ກ', 'ຂ', // 老挝语
    'A', 'b', // 英文
    '1', '2', // 数字
    '!', '。', // 符号
  ];

  println!(
    "{:<10} | {:<5} | {:<5} | {:<5} | {:<5} | {:<5} | {:<5}",
    "Character", "is_cn", "is_ja", "is_th", "is_zhtw", "is_km", "is_lo"
  );
  println!("{}", "-".repeat(60));

  for &c in &test_chars {
    println!(
      "{:<10} | {:<5} | {:<5} | {:<5} | {:<5} | {:<5} | {:<5}",
      c,
      is_cn_char(c),
      is_ja_char(c),
      is_th_char(c),
      is_zhtw_char(c),
      is_km_char(c),
      is_lo_char(c)
    );
  }
}
