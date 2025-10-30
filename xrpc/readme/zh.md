# xrpc：传输层无关的RPC框架

一个用于构建传输层无关RPC服务的框架。它提供一系列特性和结构来抽象底层通信协议，使开发者能专注于业务逻辑。

## 目录

- [设计思路](#设计思路)
- [核心概念](#核心概念)
- [使用演示](#使用演示)
- [目录结构](#目录结构)
- [RPC简史](#rpc简史)

## 设计思路

`xrpc` 的核心思想是关注点分离。一个RPC调用被分解为几个不同阶段：

1.  **请求接收与解析**：传输层（如gRPC、HTTP）接收请求并将其解析为参数。
2.  **业务逻辑执行**：执行RPC方法的核心逻辑。
3.  **响应生成**：将业务逻辑的结果转换回传输层特定的响应格式。

`xrpc` 提供 traits 来标准化第二阶段，同时为第一和第三阶段提供钩子和适配器。这使得核心应用逻辑独立于传输层。`call!` 宏自动处理日志、计时和错误处理，进一步简化了开发者的任务。

## 核心概念

`xrpc` 的主要组件在 `src/lib.rs` 中导出。

-   **`trait Func`**：定义RPC函数的基本签名，关联 `Args` 和 `Result` 类型。
-   **`trait Call` / `trait AsyncCall`**：为同步或异步RPC函数实现这些 trait。你只需定义 `inner` 方法，而 `call` 方法（由 `call!` 宏提供）会用日志、错误处理和度量来包装它。
-   **`struct ReqArgs`**：一个包装器，为 `inner` 方法提供请求上下文（包括头信息）和已解析的参数。
-   **`trait Req`**：代表传入的请求，提供 `ext` 方法以访问请求范围的扩展数据。
-   **`trait Ext`**：用于以懒加载方式提取和初始化请求范围数据的 trait，例如用户会话或数据库连接。
-   **`enum Result<T>`**：为 `inner` 方法设计的专用结果类型。它可以是：
    -   `Ok(T)`：操作成功。
    -   `Err(anyhow::Error)`：发生通用错误。框架会自动将其转换为标准的500级别错误响应。
    -   `Response(Response)`：方法需要返回一个特定的错误响应（例如，404 Not Found）。
-   **`struct Response`**：代表一个带有状态码和消息体的直接错误响应。

## 使用演示

由于项目不包含 `tests/` 目录，这里是一个如何使用 `xrpc` 和 `volo-grpc` 定义和使用RPC服务的概念性示例。

首先，在 `.proto` 文件中定义你的服务及其方法：

```proto
syntax = "proto3";

message HelloRequest {
    string name = 1;
}

message HelloResponse {
    string message = 1;
}

service Greeter {
    rpc SayHello(HelloRequest) returns (HelloResponse);
}
```

接下来，为 `SayHello` RPC实现 `AsyncCall` trait：

```rust
use xrpc::{AsyncCall, Func, ReqArgs, Result};
use your_generated_types::{HelloRequest, HelloResponse}; // 假设是由 prost/pilota 生成的类型

pub struct SayHello;

impl Func for SayHello {
    type Args = HelloRequest;
    type Result = HelloResponse;

    fn name() -> &'static str {
        "SayHello"
    }
}

impl AsyncCall for SayHello {
    async fn inner<H: Map, E: Ext>(req_args: ReqArgs<H, E, Self::Args>) -> Result<Self::Result> {
        let message = format!("Hello, {}!", req_args.args.name);
        Result::Ok(HelloResponse { message })
    }
}
```

最后，将其集成到你的 `volo-grpc` 服务实现中：

```rust
use volo_grpc::Request;
use xrpc::volo::grpc::{split, IntoResponse};
use xrpc::Call; // 根据情况使用 Call 或 AsyncCall

// 假设的服务结构体
pub struct MyService;

#[volo::async_trait]
impl Greeter for MyService {
    async fn say_hello(&self, req: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
        // 1. 将 volo 请求拆分为 xrpc 部件
        let (req, args) = split(req);
        
        // 2. 执行 xrpc 调用
        let result = SayHello::call::<MyLogger, _, _>((&req, args)).await;

        // 3. 将 xrpc 结果转换回 volo 响应
        result.into_response()
    }
}
```

## 目录结构

```
/
├── Cargo.toml       # 包配置
├── AGENTS.md        # Agent 指令
├── src/             # 源代码
│   ├── lib.rs       # 主库文件，导出公共API
│   ├── call.rs      # 定义核心的 `Call` 和 `AsyncCall` traits
│   ├── response.rs  # 定义用于直接错误返回的 `Response` 结构体
│   ├── result.rs    # 定义用于RPC结果的 `Result` 枚举
│   └── volo/        # Volo 框架的适配器
│       ├── mod.rs
│       ├── grpc.rs  # gRPC 相关辅助函数
│       └── http.rs  # HTTP 相关辅助函数
└── readme/          # README 文件
    ├── en.md        # 英文 README
    └── zh.md        # 中文 README
```

## RPC简史

远程过程调用（RPC）是分布式系统的基石。其概念起源于1970年代，最早的重要实现之一是在施乐帕洛阿尔托研究中心（Xerox PARC）。这个想法简单而强大：让远程机器上的函数调用看起来和感觉上都像本地调用一样。

1980年代，这个术语开始普及，主要归功于Sun Microsystems及其在网络文件系统（NFS）中的实现。那个时代由SunRPC（后来的ONC RPC）和DCE/RPC等技术主导。

2000年代互联网的兴起将RPC带入了HTTP，催生了XML-RPC和SOAP（简单对象访问协议）等协议。它们使用基于文本的格式（XML）进行通信，以性能为代价换取了互操作性。

在微服务浪潮的推动下，现代RPC已回归到高性能的二进制协议。像Google的gRPC（使用Protocol Buffers）和Apache Thrift等框架占据了主导地位。它们为现代分布式应用提供了必不可少的特性，如高效序列化、流处理和语言无关性，延续了1970年代最初那个想法的演进之路。
