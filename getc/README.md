[‼️]: ✏️README.mdt

# getc

```rust
use getc::getc;

#[test]
fn main() {
  let mut txtpos = tp::TxtPos::default();
  let code = r##"
// 1 引入必要的库
use std::iter::from_fn;

/*
 * 2 晚上
 * 3 天气
 * r#"4 不错"#
 *
*/

fn char_iter(s: impl AsRef<str>) -> impl Iterator<Item = (usize, char)> {
    // 5 获取字符串引用和字符索引迭代器
    let s = s.as_ref();
    let s = "// 单行字符串，不应该出现\" //'不' 不";
    let s = r#"
    "  // 多行字符串，不应该出现
    " 不 " // 不
    "#;
}

// 6 最后的注释"##;

  getc("rust", code, &mut txtpos);

  for i in txtpos.pos_li {
    println!("{:?}", txtpos.txt_li[i]);
  }
  dbg!(&txtpos.txt_li);
  assert_eq!(code, txtpos.txt_li.join(""));
}
```
