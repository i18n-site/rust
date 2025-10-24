# xrpc

[English](#en) | [中文](#cn)

<a id="en"></a>

## Table of Contents
- [Introduction](#en-introduction)
- [Usage](#en-usage)
- [Design](#en-design)
- [Tech Stack](#en-tech-stack)
- [Directory Structure](#en-directory-structure)
- [History](#en-history)

<a id="en-introduction"></a>

## Introduction

xrpc is a Rust library that provides unified interfaces for handling requests from different RPC frameworks. It defines common traits for request objects, allowing code to work with various RPC implementations through a consistent API.

<a id="en-usage"></a>

## Usage

The library provides `Req` and `Map` traits that can be implemented by RPC request types. Implementations are provided for `volo-grpc` and `volo-http` request types when the respective features are enabled.

Example:
```rust
use xrpc::Req;

fn handle_request<T: Req>(req: T) {
    let headers = req.headers();
    // Process headers and request data
}
```

<a id="en-design"></a>

## Design

The library defines two core traits:
- `Map`: Interface for accessing key-value data (e.g., headers)
- `Req`: Interface for accessing request data including headers and extensions

Implementations of these traits are provided for different RPC frameworks through conditional compilation features.

<a id="en-tech-stack"></a>

## Tech Stack

- Rust 2024 edition
- http crate for HTTP types
- volo-grpc and volo-http for RPC implementations (optional features)

<a id="en-directory-structure"></a>

## Directory Structure

```
src/
├── lib.rs          # Core traits and types
├── impl_volo_grpc.rs # volo-grpc implementation
├── impl_volo_http.rs # volo-http implementation
```

<a id="en-history"></a>

## History

The concept of adapter patterns in RPC systems can be traced back to early distributed computing systems. This library follows that tradition by providing a clean abstraction layer between business logic and RPC framework specifics.

---

<a id="cn"></a>

## 目录
- [介绍](#cn-introduction)
- [使用](#cn-usage)
- [设计](#cn-design)
- [技术栈](#cn-tech-stack)
- [目录结构](#cn-directory-structure)
- [历史](#cn-history)

<a id="cn-introduction"></a>

## 介绍

xrpc 是一个Rust库，为处理来自不同RPC框架的请求提供统一接口。它定义了请求对象的通用特性，允许代码通过一致的API与各种RPC实现协作。

<a id="cn-usage"></a>

## 使用

该库提供了`Req`和`Map`特性，可由RPC请求类型实现。当启用相应功能时，为`volo-grpc`和`volo-http`请求类型提供实现。

示例：
```rust
use xrpc::Req;

fn handle_request<T: Req>(req: T) {
    let headers = req.headers();
    // 处理头部和请求数据
}
```

<a id="cn-design"></a>

## 设计

该库定义了两个核心特性：
- `Map`：访问键值数据的接口（例如，头部）
- `Req`：访问请求数据的接口，包括头部和扩展

通过条件编译功能为不同的RPC框架提供这些特性的实现。

<a id="cn-tech-stack"></a>

## 技术栈

- Rust 2024 版本
- http crate 用于HTTP类型
- volo-grpc 和 volo-http 用于RPC实现（可选功能）

<a id="cn-directory-structure"></a>

## 目录结构

```
src/
├── lib.rs          # 核心特性和类型
├── impl_volo_grpc.rs # volo-grpc 实现
├── impl_volo_http.rs # volo-http 实现
```

<a id="cn-history"></a>

## 历史

RPC系统中的适配器模式概念可以追溯到早期的分布式计算系统。该库遵循这一传统，通过在业务逻辑和RPC框架细节之间提供一个清晰的抽象层。

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