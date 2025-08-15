# 中文繁简体转换 / Traditional and Simplified Chinese conversion

基于 [daachorse](https://github.com/daac-tools/daachorse) 实现快速替换，效率高

转换表经过精修，效果好

```rust
use cnfj::{f2j, j2f};

fn get_test_cases() -> Vec<(&'static str, &'static str)> {
  vec![
    ("河水都乾涸了", "河水都干涸了"),
    ("計劃度假", "计划度假"),
    ("憂鬱的烏龜", "忧郁的乌龟"),
    ("發財", "发财"),
    ("我是一個正體字。", "我是一个正体字。"),
    ("你好世界", "你好世界"),
    ("", ""),
    ("abc 123", "abc 123"),
    ("龍", "龙"),
    ("蘋果", "苹果"),
    ("理髮", "理发"),
    ("發現", "发现"),
    ("皇后", "皇后"),
    ("後面", "后面"),
    ("乾淨", "干净"),
    ("幹部", "干部"),
    ("我只愛你", "我只爱你"),
    ("醜陋", "丑陋"),
    ("子丑寅卯", "子丑寅卯"),
    ("歷史", "历史"),
    ("日曆", "日历"),
    ("游泳", "游泳"),
    ("旅遊", "旅游"),
    ("舞臺", "舞台"),
    ("颱風", "台风"),
    ("家裡", "家里"),
    ("公里", "公里"),
  ]
}

#[test]
fn test_f2j() {
  for (traditional, simplified) in get_test_cases() {
    assert_eq!(f2j(traditional), simplified);
  }
  assert_eq!(f2j("一隻"), "一只");
}

#[test]
fn test_j2f() {
  for (traditional, simplified) in get_test_cases() {
    assert_eq!(j2f(simplified), traditional);
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
