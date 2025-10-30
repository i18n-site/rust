# 中文繁简体转换 / Traditional and Simplified Chinese conversion

基于 [daachorse](https://github.com/daac-tools/daachorse) 实现快速替换，效率高

转换表经过精修，效果好

```rust
use cnfj::{f2j, j2f};

fn get_test_cases() -> Vec<(&'static str, &'static str)> {
  vec![
    // === No Conversion Needed ===
    // Empty, ASCII, and characters that are the same in both Traditional and Simplified
    ("", ""),
    ("abc 123", "abc 123"),
    ("你好世界", "你好世界"),
    ("子丑寅卯", "子丑寅卯"),
    ("公里", "公里"),
    ("游泳", "游泳"),
    ("名著", "名著"),
    ("皇后", "皇后"), // 后 is not converted to 後 in this context
    ("瞭望", "瞭望"), // 瞭 is the same
    // === Common Word Mappings ===
    ("家裡", "家里"),
    ("後面", "后面"),
    ("旅遊", "旅游"),
    ("日曆", "日历"),
    ("歷史", "历史"),
    ("理髮", "理发"),
    ("舞臺", "舞台"),
    ("蘋果", "苹果"),
    ("醜陋", "丑陋"),
    ("颱風", "台风"),
    ("麺包", "面包"),
    ("輕鬆", "轻松"),
    ("電視", "电视"),
    ("電腦", "电脑"),
    ("項目", "项目"),
    // === Context-Dependent Mappings (Polyphonic Characters) ===
    // 乾/干
    ("乾淨", "干净"),
    ("乾涸", "干涸"),
    ("乾杯", "干杯"),
    // 幹/干
    ("幹活", "干活"),
    ("幹部", "干部"),
    ("樹幹", "树干"),
    // 發/发
    ("發現", "发现"),
    ("發財", "发财"),
    ("頭髮", "头发"),
    // 著/着
    ("著急", "着急"),
    ("著名", "著名"), // No conversion for this '著'
    ("只有", "只有"), // No conversion for this '只'
    // === Phrases and Sentences ===
    ("憂鬱的烏龜", "忧郁的乌龟"),
    ("我只愛你", "我只爱你"),
    ("我是一個正體字。", "我是一个正体字。"),
    ("河水都乾涸了", "河水都干涸了"),
    ("計劃度假", "计划度假"),
    ("繁體中文", "繁体中文"),
    ("滑鼠和鍵盤", "滑鼠和键盘"),
  ]
}

// #[test]
// fn test() {
//   dbg!(j2f("面包"));
//   dbg!(f2j("麺包"));
// }

#[test]
fn test_j2f() {
  for (traditional, simplified) in get_test_cases() {
    assert_eq!(
      j2f(simplified),
      traditional,
      "Failed j2f: {} -> {}",
      simplified,
      traditional
    );
  }
}

#[test]
fn test_f2j() {
  for (traditional, simplified) in get_test_cases() {
    assert_eq!(
      f2j(traditional),
      simplified,
      "Failed f2j: {} -> {}",
      traditional,
      simplified
    );
  }
  assert_eq!(f2j("一隻"), "一只");
  assert_eq!(f2j("我有一隻貓"), "我有一只猫");
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

* [i18 : MarkDown 命令行翻译工具](https://i18n.site/i18)

  翻译能够完美保持 Markdown 的格式。能识别文件的修改，仅翻译有变动的文件。

  Markdown 翻译内容可编辑；如果你修改原文并再次机器翻译，手动修改过的翻译不会被覆盖 （ 如果这段原文没有被修改 ）。

* [i18n.site : MarkDown 多语言静态站点生成器](https://i18n.site/i18n.site) 为阅读体验而优化。
