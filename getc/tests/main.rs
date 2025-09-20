use getc::getc;
use txt_li::TxtLi;

#[test]
fn main() {
  //   let code = r##"
  // // 1 引入必要的库
  // use std::iter::from_fn;
  //
  // /*
  //  * 2 晚上
  //  * 3 天气
  //  * r#"4 不错"#
  //  *
  // */
  //
  // fn char_iter(s: impl AsRef<str>) -> impl Iterator<Item = (usize, char)> {
  //     // 5 获取字符串引用和字符索引迭代器
  //     let s = s.as_ref();
  //     let s = "// 单行字符串，不应该出现\" //'不' 不";
  //     let s = r#"
  //     "  // 多行字符串，不应该出现
  //     " 不 " // 不
  //     "#;
  // }
  //
  // // 6 最后的注释"##;
  //   getc("rust", code, &mut txt_li);

  //   let code = r##"
  //
  // ignore:
  //   # 忽略以 _ 开头的所有文件
  //   - _*
  //   # 忽略以 .out 或 .log 结尾的文件
  //   - *.{out,log}
  // "##;
  //   getc("yml", code, &mut txt_li);
  //   let code = r##"#!bash
  // ignore:
  //   # 忽略以 _ 开头的所有文件
  //   - _*
  //   # 忽略以 .out 或 .log 结尾的文件
  //   - *.{out,log}
  // "##;
  // getc("yml", code, &mut txt_li);

  for (lang, code) in [
    (
      "rust",
      r##"
```rust
fn main(){
  // 注释
}
    /* 
     * 多行注释
     */
```
  "##,
    ),
    (
      "i18n",
      r##"
  #告警级别Md5
// 中文
- 生成时间：${alarm_active_at} // 测试
<div class="text-title">故障描述</div>
"text": "//分派人员：{{range .Responders}}@{{.PersonName}}{{end}}{{end}}",
事件4：es.nj.03，cpu.idle = 10%，Ok
"##,
    ),
  ] {
    let code = code.trim();
    let mut txt_li = TxtLi::new();
    getc(&mut txt_li, lang, code);

    for i in &txt_li.li {
      println!("{i}");
    }

    assert_eq!(code, txt_li.restore.load(&txt_li.li));
  }
}
