# CN Util

## 中文繁简体转换

```rust
#[test]
fn test() {
  //   let s = "B端工具型产品在企业环境中发挥着至关重要的作用，然而，它们的复杂性常常让用户感到困扰。引导式设计成为解决这一挑战的重要策略。通过本文，我们将深入探讨B端产品引导式设计的特殊性，以及如何通过它来提高效率、降低成本，满足用户需求，创造更大价值。";
  //   let f = cnu::j2f(s);
  //   let j = cnu::f2j(s);
}
```

输出

```
direnv: loading /Volumes/d/3Ti/rust/.envrc
direnv: loading /Volumes/d/3Ti/.envrc
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Volumes/d/3Ti/rust/sts/Cargo.toml
workspace: /Volumes/d/3Ti/rust/Cargo.toml
warning: /Volumes/d/3Ti/rust/cnu/Cargo.toml: unused manifest key: target.cfg(target_os = "macos").rust
   Compiling cnu v0.1.5 (/Volumes/d/3Ti/rust/cnu)
    Finished test [unoptimized + debuginfo] target(s) in 0.29s
     Running unittests src/lib.rs (/Volumes/d/3Ti/rust/target/debug/deps/cnu-b4a55d95a6f81fb9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/main.rs (/Volumes/d/3Ti/rust/target/debug/deps/main-edb42f999d2e58d0)

running 1 test
[cnu/tests/main.rs:8:3] f = "B端工具型產品在企業環境中发揮着至關重要的作用，然而，它們的复雜性常常讓用戶感到困擾。引導式設計成為解決這一挑戰的重要策略。通過本文，我們將深入探討B端產品引導式設計的特殊性，以及如何通過它來提高效率、降低成本，滿足用戶需求，創造更大價值。"
[cnu/tests/main.rs:9:3] j = "B端工具型产品在企业环境中发挥着至关重要的作用，然而，它们的复杂性常常让用户感到困扰。引导式设计成为解决这一挑战的重要策略。通过本文，我们将深入探讨B端产品引导式设计的特殊性，以及如何通过它来提高效率、降低成本，满足用户需求，创造更大价值。"
[cnu/tests/main.rs:11:3] is_cn_char('a') = false
[cnu/tests/main.rs:12:3] is_cn_char('我') = true
test test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests cnu

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## 参考资料

中文汉字和常见英文数字等的 unicode 编码范围实例

| 字符集 | 字数 | Unicode 编码 |
| - | - | -  |
| 基本汉字 | 20902 | 4E00-9FA5 |
| 基本汉字补充 | 38 | 9FA6-9FCB |
| 扩展 A | 6582 | 3400-4DB5 |
| 扩展 B | 42711 | 20000-2A6D6 |
| 扩展 C | 4149 | 2A700-2B734 |
| 扩展 D | 222 | 2B740-2B81D |
| 康熙部首 | 214 | 2F00-2FD5 |
| 部首扩展 | 115 | 2E80-2EF3 |
| 兼容汉字 | 477 | F900-FAD9 |
| 兼容扩展 | 542 | 2F800-2FA1D |
| PUA(GBK) 部件 | 81 | E815-E86F |
| 部件扩展 | 452 | E400-E5E8 |
| PUA 增补 | 207 | E600-E6CF |
| 汉字笔画 | 36 | 31C0-31E3 |
| 汉字结构 | 12 | 2FF0-2FFB |
| 汉语注音 | 22 | 3105-3120 |
| 注音扩展 | 22 | 31A0-31BA |
| 〇 | 1 | 3007 |

[Detecting Chinese Characters in Unicode Strings](https://medium.com/the-artificial-impostor/detecting-chinese-characters-in-unicode-strings-4ac839ba313a)

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