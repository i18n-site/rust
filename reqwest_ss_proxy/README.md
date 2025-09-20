# reqwest_ss_proxy

[English](#english) | [中文](#中文)

---

## English

### Table of Contents
- [Project Significance](#project-significance)
- [Tech Stack](#tech-stack)
- [Design Philosophy](#design-philosophy)
- [File Structure](#file-structure)
- [Usage](#usage)
- [A Little Story](#a-little-story)

### Project Significance

In many regions, accessing the global internet is restricted by firewalls. For developers and applications in these areas, routing traffic through a proxy is not just a convenience but a necessity. Shadowsocks (SS) has emerged as one of the most popular and effective protocols for circumventing such censorship.

This project, `reqwest-ss-proxy`, provides a `reqwest` middleware that enables Rust applications to seamlessly route their HTTP/HTTPS requests through a Shadowsocks proxy. It simplifies development for any application that needs reliable access to geo-restricted or firewalled resources, making `reqwest` a more powerful tool for global connectivity.

### Tech Stack

*   **`reqwest`**: An ergonomic, batteries-included HTTP client for Rust.
*   **`reqwest-middleware`**: A framework for creating and chaining middleware for `reqwest`.
*   **`shadowsocks-rust`**: A robust and efficient Rust implementation of the Shadowsocks protocol.
*   **`tokio`**: The de facto asynchronous runtime for network applications in Rust.
*   **`anyhow`**: For flexible and user-friendly error handling.

### Design Philosophy

The core design principle is **simplicity and seamless integration**. The middleware is intended to be a "drop-in" component for any project already using `reqwest` and `reqwest-middleware`.

It exposes a minimal and intuitive API—primarily the `SsMiddleware::from_url` function—to keep the setup process as straightforward as possible. The goal is for the middleware to be transparent in its operation; once configured, it proxies all requests without requiring any changes to the application's existing request-building logic. This allows developers to focus on their application's features rather than the complexities of proxying.

### File Structure

```
.
├── Cargo.toml       # Project manifest and dependencies
├── src
│   ├── lib.rs       # Main library entry point, exports the middleware
│   ├── reqwest.rs   # Core implementation of the SsMiddleware
│   └── error.rs     # Custom error types for the library
└── tests
    └── main.rs      # Integration tests demonstrating usage
```

### Usage

Here is a practical example of how to integrate `SsMiddleware` with `reqwest_middleware::ClientBuilder`, inspired by `tests/main.rs`.

```rust
use anyhow::Result;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_ss_proxy::SsMiddleware;

#[tokio::main]
async fn main() -> Result<()> {
  // 1. Define your Shadowsocks server URL.
  //    Replace with your actual server details.
  let url = "ss://aes-256-gcm:password@your-server-address:port";

  // 2. Create an instance of the SsMiddleware from the URL.
  let ss_middleware = SsMiddleware::from_url(url)?;

  // 3. Build a reqwest client and attach the middleware.
  //    It's good practice to disable the system proxy to ensure
  //    traffic goes exclusively through the SS middleware.
  let client: ClientWithMiddleware = ClientBuilder::new(
    reqwest::Client::builder().no_proxy().build()?
  ).with(ss_middleware).build();

  // 4. Use the client to send requests as you normally would.
  //    The traffic will be automatically proxied.
  let test_url = "https://ifconfig.me/ip";
  match client.get(test_url).send().await {
    Ok(res) => {
      let status = res.status();
      let ip = res.text().await?;
      // The IP printed should be that of your proxy server.
      println!("Request to {test_url}: Status={status}, IP={ip}");
    }
    Err(e) => {
      eprintln!("Request to {test_url} failed: {e}");
    }
  }

  Ok(())
}
```

### A Little Story

The story of Shadowsocks begins in 2012 with a programmer known as "clowwindy." It started as a personal project to create an efficient and encrypted proxy. Its core innovation was a protocol designed to be both lightweight and indistinguishable from regular HTTPS traffic, making it difficult for automated network tools to classify.

Being open-source was key to its success. A global community of developers was able to contribute, adapt, and improve upon the original design. When the original author ceased active development in 2015, the project's open nature allowed the community to carry it forward. Development continued through numerous forks and independent implementations, like the `shadowsocks-rust` crate this project uses. This history is a powerful example of how a valuable open-source tool can thrive and evolve through community collaboration.

---

## 中文

### 目录
- [项目意义](#项目意义-1)
- [技术栈](#技术栈-1)
- [设计思路](#设计思路-1)
- [文件结构](#文件结构-1)
- [使用演示](#使用演示-1)
- [相关故事](#相关故事)

### 项目意义

在许多地区，访问全球互联网受到防火墙的限制。对于这些地区的开发者和应用程序来说，通过代理路由流量不仅是为了方便，更是一种必需。Shadowsocks (SS) 已成为规避此类审查最流行和最有效的协议之一。

本项目 `reqwest-ss-proxy` 提供了一个 `reqwest` 中间件，使 Rust 应用程序能够无缝地将其 HTTP/HTTPS 请求通过 Shadowsocks 代理进行路由。它为任何需要可靠访问受地理限制或防火墙限制资源的应用程序简化了开发过程，使 `reqwest` 成为一个更强大的全球连接工具。

### 技术栈

*   **`reqwest`**: 一个符合人体工程学、功能完备的 Rust HTTP 客户端。
*   **`reqwest-middleware`**: 一个用于为 `reqwest` 创建和链接中间件的框架。
*   **`shadowsocks-rust`**: 一个健壮且高效的 Shadowsocks 协议 Rust 实现。
*   **`tokio`**: Rust 网络应用程序事实上的标准异步运行时。
*   **`anyhow`**: 用于实现灵活且用户友好的错误处理。

### 设计思路

核心设计原则是**简单性与无缝集成**。该中间件旨在成为任何已使用 `reqwest` 和 `reqwest-middleware` 的项目的“即插即用”组件。

它暴露了最小化且直观的 API——主要是 `SsMiddleware::from_url` 函数——以使设置过程尽可能简单直接。其目标是让中间件在操作上保持透明；一旦配置完成，它会代理所有请求，而无需对应用程序现有的请求构建逻辑进行任何更改。这使开发人员能够专注于其应用程序的功能，而不是代理的复杂性。

### 文件结构

```
.
├── Cargo.toml       # 项目清单和依赖项
├── src
│   ├── lib.rs       # 主库入口点，导出中间件
│   ├── reqwest.rs   # SsMiddleware 的核心实现
│   └── error.rs     # 库的自定义错误类型
└── tests
    └── main.rs      # 集成测试，演示用法
```

### 使用演示

这是一个如何将 `SsMiddleware` 与 `reqwest_middleware::ClientBuilder` 集成使用的实用示例，其灵感来自 `tests/main.rs`。

```rust
use anyhow::Result;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_ss_proxy::SsMiddleware;

#[tokio::main]
async fn main() -> Result<()> {
  // 1. 定义你的 Shadowsocks 服务器 URL。
  //    请替换为你的实际服务器信息。
  let url = "ss://aes-256-gcm:password@your-server-address:port";

  // 2. 从 URL 创建一个 SsMiddleware 实例。
  let ss_middleware = SsMiddleware::from_url(url)?;

  // 3. 构建一个 reqwest 客户端并附加中间件。
  //    禁用系统代理是一个好习惯，以确保流量完全通过 SS 中间件。
  let client: ClientWithMiddleware = ClientBuilder::new(
    reqwest::Client::builder().no_proxy().build()?
  ).with(ss_middleware).build();

  // 4. 像平常一样使用客户端发送请求。
  //    流量将被自动代理。
  let test_url = "https://ifconfig.me/ip";
  match client.get(test_url).send().await {
    Ok(res) => {
      let status = res.status();
      let ip = res.text().await?;
      // 打印出的 IP 应该是你的代理服务器的 IP。
      println!("请求 {test_url}: Status={status}, IP={ip}");
    }
    Err(e) => {
      eprintln!("请求 {test_url} 失败: {e}");
    }
  }

  Ok(())
}
```

### 相关故事

Shadowsocks 的故事始于 2012 年，由一位名叫“clowwindy”的程序员开启。它最初是一个个人项目，旨在创建一个高效的加密代理。其核心创新在于设计了一种轻量级协议，且其流量特征与普通的 HTTPS 流量难以区分，这使得自动化网络工具很难对其进行分类。

开源是其成功的关键。一个全球性的开发者社区得以围绕它形成，不断贡献、调整和改进最初的设计。当原作者在 2015 年停止积极开发时，该项目的开放性让社区能够继承其成果并继续前进。开发通过众多的分支和独立实现得以延续，例如本项目所使用的 `shadowsocks-rust` crate。这段历史有力地证明了一个有价值的开源工具如何通过社区协作而蓬勃发展和不断演进。

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
