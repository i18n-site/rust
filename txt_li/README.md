# txt_li

```rust
use aok::{OK, Void};
use tracing::info;
use txt_li::TxtLi;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[test]
fn test_restore() -> Void {
  let mut txt_li = TxtLi::new();
  txt_li.push_md_line("## abc");
  txt_li.push_md_line("-");
  txt_li.push_md_line("  [");
  txt_li.push_md_line("  + -987");
  txt_li.push_md_line("******");
  txt_li.push_md_line("_____");
  txt_li.push_md_line("----");
  txt_li.push_md_line("-5+1");
  txt_li.push_md_line("- [x] efg");
  txt_li.push_md_line("- [ ] hlq");
  txt_li.push_md_line("- [ ]");
  txt_li.push_md_line("*. abc");
  txt_li.push_md_line("**abc**");
  txt_li.push_md_line("[ ]");
  txt_li.push_md_line("[ ] abc");
  txt_li.push_md_line("[^bignote]:");
  txt_li.push_md_line("[^bignote]:xyz");
  txt_li.push_md_line("1. ");
  txt_li.push_md_line("1.");
  txt_li.push_md_line("1. 测试");
  txt_li.push_md_line("| 表头1 | 表头2 |");
  txt_li.push_md_line("| <a> 表头1 | 表头2 |</a> |");
  txt_li.push_md_line(r"表头1 | 表头2 \| 123 | 表头3");
  txt_li.push_md_line(r"[![License](https://img.shields.io/crates/l/volo)](#license)");
  txt_li.push_md_line(r"![License](https://img.shields.io/crates/l/volo)");
  txt_li.push_md_line(r"![License]()");
  txt_li.push_md_line(r"![License](");
  txt_li.push_md_line(r"![](https://img.shields.io/crates/l/volo)");
  txt_li.push_md_line(r"[1]: https://img.shields.io");
  txt_li.push_md_line(r"[1] https://img.shields.io");
  txt_li.push_md_line(r"`测试一下`");
  txt_li.push_md_line(r"<code>测试一下</code>");
  txt_li.push_md_line(r"***");
  txt_li.push_md_line(r"**");
  txt_li.push_md_line(r"*");
  txt_li.push_md_line(r"[SurrealDB-url]: https://surrealdb.com/");
  txt_li.push_md_line(r"<!-- 测试 -->");
  txt_li.push_md_line(
    r"- 🌐 **abc No Vendor Lock-in**: Switch providers, deploy anywhere, own your data",
  );
  txt_li.push_md_line(r"[![Forks][forks-shield]][forks-url]");
  txt_li.push_md_line(r"**Cost**");
  txt_li.push_md_line(r"- [x] **Cost**");
  txt_li.push_md_line(r"·");
  txt_li.push_md_line(" (");
  txt_li.push_md_line("(");
  txt_li.push_md_line("()");
  txt_li.push_md_line("<p align=\"right\">(<a href=\"#readme-top\">back to top</a>)</p>");
  txt_li.push_md_line("<p align=\"right\"></p>");
  // dbg!(&txt_li.li);
  // info!("{}", &txt_li.restore.load(&txt_li.li));

  for i in txt_li.li {
    info!("{:?}", i);
  }
  OK
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
