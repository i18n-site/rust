[English](#en) / [中文](#cn)

<a id="en"></a>
# xbin

## Table of Contents
* [Introduction](#introduction-en)
* [Usage Demonstration](#usage-en)
* [Design Principles](#design-en)
* [Technology Stack](#tech-stack-en)
* [Directory Structure](#directory-en)
* [Historical Anecdote](#history-en)

<a id="introduction-en"></a>
## Introduction
`xbin` is a Rust library providing utilities for byte array manipulation. It focuses on efficient concatenation of various byte-like types into a single `Vec<u8>`.

<a id="usage-en"></a>
## Usage Demonstration
The library offers both a function and a macro for concatenation.

### Function Usage
The `concat` function accepts an iterator of items that implement `AsRef<[u8]>`, making it suitable for dynamic collections of byte sources:
```rust
pub fn concat<T: AsRef<[u8]>>(iter: impl IntoIterator<Item = T>) -> Vec<u8> {
  let mut r = Vec::new();
  for i in iter {
    r.extend(i.as_ref());
  }
  r
}
```

### Macro Usage
The `concat` macro provides a more convenient syntax for combining a fixed number of byte sources at compile time:
```rust
#[macro_export]
macro_rules! concat {
  ($($i:expr),*$(,)?)=>{
    [
      $($i.as_ref()),*
    ].concat()
  }
}
```

### Example
```rust
use aok::{OK, Result};
use log::info;
use static_init::constructor;
use xbin::concat;

#[constructor(0)]
extern "C" fn init() {
  log_init::init()
}

#[test]
fn test() -> Result<()> {

  let s1 = "123";
  let s2 = [4u8, 5, 6];
  let s3 = vec![7u8, 8, 9];
  let result = concat!(s1, s2, s3);
  assert_eq!(result, b"123\x04\x05\x06\x07\x08\t");

  info!("test ok");

  OK
}
```

<a id="design-en"></a>
## Design Principles
The core design centers on the `AsRef<[u8]>` trait, which enables flexible input types for concatenation. This trait is implemented by common byte-like types such as `&str`, `[u8; N]`, `Vec<u8>`, and `&[u8]`, allowing the library to handle diverse data sources uniformly. The function provides iterative processing for dynamic collections, while the macro offers compile-time efficiency for fixed arguments.

<a id="tech-stack-en"></a>
## Technology Stack
*   **Language:** Rust
*   **Core Dependencies:** None (the library itself has no external dependencies)
*   **Test Dependencies:** `aok` (for error handling in tests), `log` (for logging in tests), `static_init` (for test initialization)

<a id="directory-en"></a>
## Directory Structure
```
.
├── Cargo.toml        # Project manifest
├── README.mdt        # Project documentation
├── test.sh           # Shell script for testing (if any)
├── src/              # Source code directory
│   └── lib.rs        # Core library implementation
└── tests/            # Integration tests
    └── main.rs       # Test cases
```

<a id="history-en"></a>
## Historical Anecdote
The evolution of byte manipulation in programming languages reflects the ongoing balance between performance and safety. In early systems programming languages like C, developers had direct control over memory, enabling highly optimized byte operations but also introducing risks like buffer overflows. As software complexity grew, the need for safer abstractions became apparent. Rust emerged to address these challenges by providing zero-cost abstractions like the `AsRef<[u8]>` trait, which enables efficient byte operations while maintaining memory safety guarantees. The `xbin` library exemplifies this approach, offering a clean interface for byte concatenation that leverages Rust's trait system to work with multiple input types without sacrificing performance.

---

<a id="cn"></a>
# xbin

## 目录
* [项目介绍](#introduction-cn)
* [使用演示](#usage-cn)
* [设计思路](#design-cn)
* [技术栈](#tech-stack-cn)
* [目录结构](#directory-cn)
* [技术轶事](#history-cn)

<a id="introduction-cn"></a>
## 项目介绍
`xbin` 是一个 Rust 库，提供字节数组处理工具。其核心功能是高效地将多种字节类型连接成一个 `Vec<u8>`。

<a id="usage-cn"></a>
## 使用演示
该库提供函数和宏两种方式进行连接操作。

### 函数用法
`concat` 函数接受实现 `AsRef<[u8]>` trait 的迭代器，适用于动态字节源集合：
```rust
pub fn concat<T: AsRef<[u8]>>(iter: impl IntoIterator<Item = T>) -> Vec<u8> {
  let mut r = Vec::new();
  for i in iter {
    r.extend(i.as_ref());
  }
  r
}
```

### 宏用法
`concat` 宏在编译时提供更便捷的语法，用于组合固定数量的字节源：
```rust
#[macro_export]
macro_rules! concat {
  ($($i:expr),*$(,)?)=>{
    [
      $($i.as_ref()),*
    ].concat()
  }
}
```

### 示例
```rust
use aok::{OK, Result};
use log::info;
use static_init::constructor;
use xbin::concat;

#[constructor(0)]
extern "C" fn init() {
  log_init::init()
}

#[test]
fn test() -> Result<()> {

  let s1 = "123";
  let s2 = [4u8, 5, 6];
  let s3 = vec![7u8, 8, 9];
  let result = concat!(s1, s2, s3);
  assert_eq!(result, b"123\x04\x05\x06\x07\x08\t");

  info!("test ok");

  OK
}
```

<a id="design-cn"></a>
## 设计思路
核心设计围绕 `AsRef<[u8]>` trait，实现灵活的输入类型连接。此 trait 被常见字节类型如 `&str`、`[u8; N]`、`Vec<u8>` 和 `&[u8]` 实现，使库能够统一处理不同数据源。函数提供动态集合的迭代处理，宏提供固定参数的编译时效率。

<a id="tech-stack-cn"></a>
## 技术栈
*   **语言:** Rust
*   **核心依赖:** 无 (库本身无外部依赖)
*   **测试依赖:** `aok` (测试错误处理), `log` (测试日志), `static_init` (测试初始化)

<a id="directory-cn"></a>
## 目录结构
```
.
├── Cargo.toml        # 项目清单
├── README.mdt        # 项目文档
├── test.sh           # 测试脚本 (如果有)
├── src/              # 源代码目录
│   └── lib.rs        # 核心库实现
└── tests/            # 集成测试
    └── main.rs       # 测试用例
```

<a id="history-cn"></a>
## 技术轶事
编程语言中字节操作的演进反映了性能与安全性之间的持续平衡。在 C 等早期系统编程语言中，开发者对内存有直接控制权，能够实现高度优化的字节操作，但也引入了缓冲区溢出等风险。随着软件复杂性增长，对更安全抽象的需求变得明显。Rust 的出现解决了这些挑战，通过 `AsRef<[u8]>` 等零成本抽象，在保持内存安全保证的同时提供高效字节操作。`xbin` 库体现了这种方法，利用 Rust trait 系统为多种输入类型提供简洁接口，而不牺牲性能。

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