xxx.xx.com 重定向到xx.com

# Rust 反向代理服务器

基于 Rust 2024 edition 开发的高性能反向代理服务器。

## 功能特性

✅ **已实现功能**
- [x] 基于 tokio 的异步反向代理
- [x] 支持 HTTP/1.1 和 HTTP/2 协议
- [x] TLS 1.2 和 1.3 支持 (使用 rustls)
- [x] 轮询负载均衡算法
- [x] 连接超时和请求超时控制
- [x] 失败重试机制 (最多3次)
- [x] 证书过期时间监控
- [x] 优雅重启支持 (SIGUSR1)
- [x] 优雅关闭支持 (SIGTERM/SIGINT)
- [x] 结构化日志记录 (tracing)
- [x] 错误处理 (thiserror)

✅ **已完成功能**

1. **证书自动管理系统**
   - ✅ 自动解析证书过期时间
   - ✅ 按过期时间排序存储
   - ✅ 每天检查即将过期的证书
   - ✅ 如果过期时间小于3天，自动重新加载证书
   - ✅ CertManager 添加 LoadCert trait，支持 `CertManager<T: LoadCert>`
   - ✅ 支持多域名证书，使用最短域名作为标识
   - ✅ 提供 FileCertLoader 实现，支持从文件系统自动重新加载证书

2. **HTTP/3 支持**
   - ✅ HTTP/3 基础框架已完成
   - ⚠️ HTTP/3 完整实现暂时禁用（由于 h3 crate API 兼容性问题）
   - 📝 当前版本会启动 HTTP/3 服务器但仅等待关闭信号

**新增特性**

### LoadCert Trait
```rust
pub trait LoadCert: Send + Sync {
  fn load(&self, domain: &str) -> impl Future<Output = Result<Cert>> + Send;
}
```

### 证书管理器泛型支持
```rust
// 使用默认加载器（不支持自动重新加载）
let cert_manager = CertManager::new(DefaultCertLoader);

// 使用文件加载器（支持自动重新加载）
let mut file_loader = FileCertLoader::new();
file_loader.add_cert_path("example.com".to_string(), "cert.pem".to_string(), "key.pem".to_string());
let cert_manager = CertManager::new(file_loader);
```

### 自动证书重新加载
- 每天检查证书过期时间
- 过期时间小于3天时自动触发重新加载
- 支持多域名证书的智能域名匹配
- 线程安全的证书更新机制


## 使用方法

### 基本使用

```bash
# 构建项目
cargo build --release --examples

# 启动测试后端服务器 (在另一个终端)
./run_backend.sh
# 或者
cargo run --example test_backend

# 运行反向代理服务器
./run.sh
# 或者
cargo run --example basic

# 测试 HTTP/1.1
curl -H "Host: 018007.xyz" http://localhost:8080/

# 测试 HTTPS/HTTP2 (忽略证书验证)
curl -k -H "Host: 018007.xyz" https://localhost:8443/

# 优雅重启 (发送 SIGUSR1 信号)
kill -USR1 <pid>

# 优雅关闭 (发送 SIGTERM 信号)
kill -TERM <pid>
```

### 配置说明

服务器配置在 `src/conf.rs` 中定义：

```rust
pub struct Conf {
    pub http: SocketAddr,    // HTTP/1.1 监听地址
    pub h2: SocketAddr,      // HTTP/2 监听地址 (TLS)
    pub h3: SocketAddr,      // HTTP/3 监听地址 (QUIC)
    pub route: Route,    // 站点映射配置
}
```

## 架构设计

### 模块结构

- `cert.rs` - 证书管理和过期监控
- `conf.rs` - 配置定义
- `error.rs` - 错误类型定义
- `graceful.rs` - 优雅重启和关闭
- `h3.rs` - HTTP/3 服务器 (基于 quinn)
- `load_balancer.rs` - 负载均衡算法
- `proxy.rs` - HTTP 代理处理逻辑
- `server.rs` - 主服务器实现

### 负载均衡

使用轮询算法，支持：
- 连接超时检测 (3秒)
- 请求超时控制 (120秒)
- 失败重试 (最多3次)
- 健康检查

## 性能特性

- 异步 I/O (tokio)
- 零拷贝数据传输
- 连接池复用
- 内存高效的数据结构

## 安全特性

- TLS 1.2/1.3 支持
- ECC 证书支持
- 安全的头部处理
- 输入验证和清理

## 监控和日志

使用 `tracing` 框架提供结构化日志：
- 请求/响应日志
- 性能指标
- 错误追踪
- 连接状态监控

## 开发状态

当前版本实现了核心的反向代理功能，包括 HTTP/1.1、HTTP/2 支持，负载均衡，优雅重启等特性。HTTP/3 支持的基础框架已完成，但需要进一步完善。

### 示例配置

- 使用真实的 ECC 证书 (018007.xyz)
- 支持主域名和通配符域名
- 配置了三个测试后端服务器
- 使用非特权端口便于开发测试

### 新增示例

#### 证书自动重新加载演示
```bash
# 运行证书自动重新加载演示
cargo run --example cert_reload_demo
```

此示例展示了：
- 证书管理器的自动重新加载功能
- FileCertLoader 的使用方法
- 证书过期时间监控
- 自动证书更新机制

## 依赖管理

所有依赖通过 `cargo add` 安装，按需启用特性，避免使用 `full` 特性包。
