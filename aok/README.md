[English](#en) / [中文](#cn)

<a id="en"></a>
# aok

## Table of Contents
- [Introduction](#introduction-en)
- [Usage Example](#usage-example-en)
- [Design Concept](#design-concept-en)
- [Directory Structure](#directory-structure-en)
- [Historical Story](#historical-story-en)

<a id="introduction-en"></a>
## Introduction

`aok` simplifies error handling in Rust by wrapping `anyhow` crate. It provides convenient macros and type aliases to reduce boilerplate code when dealing with `Result` types.

<a id="usage-example-en"></a>
## Usage Example

```rust
use aok::{OK, Void, err, throw};

fn decode(a: u8) -> Void {
  let _ = String::from_utf8(vec![a])?;
  OK
}

fn example_err() -> Void {
  err!("This is an error")
}

fn example_throw() -> Void {
  throw!("This is a thrown error");
}

fn example_throw_with_args() -> Void {
  throw!("Error with value: {}", 42);
}

#[test]
fn test() {
  assert!(decode(99).is_ok());
  assert!(decode(128).is_err());
  assert!(example_err().is_err());
  assert!(example_throw().is_err());
  assert!(example_throw_with_args().is_err());
}
```

<a id="design-concept-en"></a>
## Design Concept

The library exports all items from `anyhow` and defines:
- `OK`: A constant representing `Ok(())`
- `Void`: Type alias for `Result<()>`
- `err!`: Macro to create an `Err` variant with `anyhow!`
- `throw!`: Macro that includes `return` to directly exit the function, supporting formatted strings and error chaining

The call flow typically involves using these macros and constants to streamline error propagation and creation within functions that return `Result` types.

Examples of macro usage:
- `err!("Message")` - Creates an error result but doesn't return from the function
- `throw!("Message")` - Creates an error and immediately returns from the function
- `throw!("Message: {}", value)` - Creates a formatted error and immediately returns

<a id="directory-structure-en"></a>
## Directory Structure

```
src/
└── lib.rs       # Main library code
tests/
└── main.rs      # Test cases
```

<a id="historical-story-en"></a>
## Historical Story

Rust's error handling has evolved significantly. Early Rust versions required extensive boilerplate for error propagation. The introduction of the `?` operator simplified this. Libraries like `anyhow` further abstracted error handling for application-level code. This project builds on those foundations to minimize setup overhead.

The `throw!` macro creates an error and immediately returns from the function. It combines error creation and function exit in a single macro call, eliminating the need for explicit return statements.

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

<a id="cn"></a>
# aok

## 目录
- [项目介绍](#introduction-cn)
- [使用演示](#usage-example-cn)
- [设计思路](#design-concept-cn)
- [目录结构](#directory-structure-cn)
- [历史故事](#historical-story-cn)

<a id="introduction-cn"></a>
## 项目介绍

`aok` 是对 `anyhow` crate 的封装，用于简化 Rust 中的错误处理。它提供了便捷的宏和类型别名，以减少处理 `Result` 类型时的样板代码。

<a id="usage-example-cn"></a>
## 使用演示

```rust
use aok::{OK, Void, err, throw};

fn decode(a: u8) -> Void {
  let _ = String::from_utf8(vec![a])?;
  OK
}

fn example_err() -> Void {
  err!("这是一个错误")
}

fn example_throw() -> Void {
  throw!("这是一个抛出的错误");
}

fn example_throw_with_args() -> Void {
  throw!("带参数的错误: {}", 42);
}

#[test]
fn test() {
  assert!(decode(99).is_ok());
  assert!(decode(128).is_err());
  assert!(example_err().is_err());
  assert!(example_throw().is_err());
  assert!(example_throw_with_args().is_err());
}
```

<a id="design-concept-cn"></a>
## 设计思路

该库导出 `anyhow` 的所有项，并定义：
- `OK`：表示 `Ok(())` 的常量
- `Void`：`Result<()>` 的类型别名
- `err!`：创建带有 `anyhow!` 的 `Err` 变体的宏
- `throw!`：返回 `Err` 变体的宏，支持格式化字符串和错误链

调用流程通常涉及使用这些宏和常量来简化函数内错误传播和创建，特别是那些返回 `Result` 类型的函数。

宏使用示例：
- `err!("消息")` - 创建错误结果但不从函数返回
- `throw!("消息")` - 创建错误并立即从函数返回
- `throw!("消息: {}", 值)` - 创建格式化错误并立即返回

<a id="directory-structure-cn"></a>
## 目录结构

```
src/
└── lib.rs       # 主库代码
tests/
└── main.rs      # 测试用例
```

<a id="historical-story-cn"></a>
## 历史故事

Rust 的错误处理机制经历了显著演变。早期版本需要大量样板代码进行错误传播。`?` 操作符的引入简化了这一过程。像 `anyhow` 这样的库进一步抽象了应用程序级别的错误处理。本项目在此基础上构建，以最小化设置开销。

`throw!` 宏用于创建错误并立即从函数返回。它将错误创建和函数退出结合在单个宏调用中，无需显式返回语句。

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