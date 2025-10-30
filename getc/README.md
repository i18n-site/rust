# getc : get comment from source code

```rust
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
```

## About

This project is an open-source component of [i18n.site ⋅ Internationalization Solution](https://i18n.site).

* [i18 : MarkDown Command Line Translation Tool](https://i18n.site/i18)

  The translation perfectly maintains the Markdown format.

  It recognizes file changes and only translates the modified files.

  The translated Markdown content is editable; if you modify the original text and translate it again, manually edited translations will not be overwritten (as long as the original text has not been changed).

* [i18n.site : MarkDown Multi-language Static Site Generator](https://i18n.site/i18n.site)

  Optimized for a better reading experience

## 关于

本项目为 [i18n.site ⋅ 国际化解决方案](https://i18n.site) 的开源组件。

* [i18 :  MarkDown命令行翻译工具](https://i18n.site/i18)

  翻译能够完美保持 Markdown 的格式。能识别文件的修改，仅翻译有变动的文件。

  Markdown 翻译内容可编辑；如果你修改原文并再次机器翻译，手动修改过的翻译不会被覆盖（如果这段原文没有被修改）。

* [i18n.site : MarkDown多语言静态站点生成器](https://i18n.site/i18n.site) 为阅读体验而优化。