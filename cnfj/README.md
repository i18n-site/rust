# 中文繁简体转换

```rust
use cnfj::{f1j, j2f};

#[test]
fn test_f2j() {
  assert_eq!(f2j("河水都乾涸了"), "河水都干涸了");
  assert_eq!(f2j("計畫渡假"), "计划度假");
  assert_eq!(f2j("憂鬱的烏龜"), "忧郁的乌龟");
  assert_eq!(f2j("發財"), "发财");
  assert_eq!(f2j("我是一個正體字。"), "我是一个正体字。");
  assert_eq!(f2j("你好世界"), "你好世界");
  assert_eq!(f2j(""), "");
  assert_eq!(f2j("abc 123"), "abc 123");
  assert_eq!(f2j("龍"), "龙");
}

#[test]
fn test_j2f() {
  assert_eq!(j2f("河水都干涸了"), "河水都乾涸了");
  assert_eq!(j2f("计划度假"), "計畫渡假");
  assert_eq!(j2f("忧郁的乌龟"), "憂鬱的烏龜");
  assert_eq!(j2f("发财"), "發財");
  assert_eq!(j2f("我是一个正体字。"), "我是一個正體字。");
  assert_eq!(j2f("你好世界"), "你好世界");
  assert_eq!(j2f(""), "");
  assert_eq!(j2f("abc 123"), "abc 123");
  assert_eq!(j2f("龙"), "龍");
}
```

输出

```
   Compiling cnfj v0.1.16 (/Users/z/i18n/rust/cnfj)
warning: unused import: `MatchKind`
  --> cnfj/src/lib.rs:10:49
   |
10 | use daachorse::{CharwiseDoubleArrayAhoCorasick, MatchKind};
   |                                                 ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused import: `static_init::dynamic`
  --> cnfj/src/lib.rs:11:5
   |
11 | use static_init::dynamic;
   |     ^^^^^^^^^^^^^^^^^^^^

warning: function `replace_with_dict` is never used
  --> cnfj/src/lib.rs:34:4
   |
34 | fn replace_with_dict(
   |    ^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: `cnfj` (lib) generated 3 warnings (3 duplicates)
warning: `cnfj` (lib test) generated 3 warnings (run `cargo fix --lib -p cnfj --tests` to apply 2 suggestions)
error[E0432]: unresolved imports `cnfj::f2j`, `cnfj::j2f`
  --> cnfj/tests/main.rs:1:11
   |
 1 | use cnfj::{f2j, j2f};
   |           ^^^  ^^^ no `j2f` in the root
   |           |
   |           no `f2j` in the root
   |           help: a similar name exists in the module: `fj`
   |
note: found an item that was configured out
  --> /Users/z/i18n/rust/cnfj/src/lib.rs:5:9
   |
 4 | #[cfg(feature = "f2j")]
   |       --------------- the item is gated behind the `f2j` feature
 5 | pub mod f2j;
   |         ^^^
note: found an item that was configured out
  --> /Users/z/i18n/rust/cnfj/src/lib.rs:51:8
   |
50 | #[cfg(feature = "f2j")]
   |       --------------- the item is gated behind the `f2j` feature
51 | pub fn f2j(text: impl AsRef<str>) -> String {
   |        ^^^
note: found an item that was configured out
  --> /Users/z/i18n/rust/cnfj/src/lib.rs:56:8
   |
55 | #[cfg(feature = "j2f")]
   |       --------------- the item is gated behind the `j2f` feature
56 | pub fn j2f(text: impl AsRef<str>) -> String {
   |        ^^^

For more information about this error, try `rustc --explain E0432`.
error: could not compile `cnfj` (test "main") due to 1 previous error
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
