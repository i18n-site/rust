# tsfmt

```rust
use chrono::NaiveDate;
use tsfmt::utc;

fn is_valid_date(date_str: &str, ts: u64) -> bool {
  match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
    Ok(_) => true,
    Err(_) => {
      eprintln!("ts {ts} INVALID DATE {}", date_str);
      false
    }
  }
}

#[test]
fn main() {
  let now = sts::sec();

  let mut n = 0;
  while n < 86400 * 373 * 100 {
    n += 3600;
    let ts = now + n;
    let s = utc(ts);
    let date = s.split("T").next().unwrap();
    assert!(is_valid_date(date, ts));
  }
  println!("\n{}\n", utc(now));
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