# mysql macro: mysql macro for mysql_async

```rust
#[tokio::test]
async fn main() -> aok::Result<()> {
  loginit::init();

  // let sql = format!("SELECT {}", 1);
  // let r: u64 = q1!(sql);
  // e(sql.clone(), vec![]).await?;

  // tracing::debug!("test");
  // let id_li = vec![1];
  // let li: HashMap<_, String> = id_v("payBrand", id_li).await?;
  // let li: HashMap<_, String> = id_v("payBrand", *&[1]).await?;

  // let mail_id: Option<Option<u64>> = q01!(r#"select mailId("a@b.c")"#);
  // let mail_id: Option<u64> = q1!(r#"select mailId("a@b.c")"#);
  //
  // e!(r"select mailHostid('a.com')");
  //
  // let mut conn = conn!();
  //
  // let mail_host_id: u64 = q1!(&mut conn; r"select mailHostid('a.com')");
  // let mail_host_id: u64 = q1!(r"select mailHostid('a.com')");
  // let mail_host_id: u64 = q1!(r"select mailHostid(?)", "a.com");
  // let mail_host_id: Option<u64> = q01!("select mailHostid('a.com')",);
  //
  // let q: Vec<Option<u64>> = q!(&mut conn; r"select mailHostid(?)","a.com");
  //
  // let q: Vec<(u64,)> = q!(r"select mailHostid(?)", "a.com");
  //
  // let q: Vec<u64> = q!(r"select mailHostid(?)", "a.com");

  // let s = r#"'\'test\''"#;
  // println!("{}", mysql_macro::s(s));
  //
  // let s = [211, 222, 223, 224, 225];
  // println!("{}", mysql_macro::b(&s[..]));
  Ok(())
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