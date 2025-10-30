use anyhow::{anyhow, Result};
use leaf::proxy::{self, OutboundHandler, ProxyStream, UdpOutboundHandler};
use leaf::session::{Session, SocksAddr};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    // 目标：不依赖外部转换器，直接通过 VLESS 链接访问一个网址。
    // 您的 VLESS 链接
    let vless_uri = "vless://87f07f63-39d3-4cb7-8188-2a5811a7aefa@146.235.228.34:443?type=tcp&encryption=none&host=&path=&headerType=none&quicSecurity=none&serviceName=&security=tls&flow=xtls-rprx-vision&fp=ios&sni=dcpqjs.yydjc.top#%F0%9F%87%BA%F0%9F%87%B8%E7%BE%8E%E5%9B%BD%E5%9C%A3%E4%BD%95%E5%A1%9E05%20%7C%20%E4%B8%89%E7%BD%91%E6%8E%A8%E8%8D%90";

    // 要访问的目标网站
    let target_host = "httpbin.org";
    let target_port = 80;

    println!("正在解析 VLESS 链接...");

    // 1. 解析 VLESS URI 并创建出站处理器 (OutboundHandler)
    let outbound_handler = parse_vless_uri_and_create_handler(vless_uri)?;

    println!("VLESS 代理配置完成，准备建立连接...");
    println!("目标网站: {}:{}", target_host, target_port);

    // 2. 创建一个虚拟的会话 (Session) 来告诉 leaf 我们想访问哪里
    let session = Session {
        destination: SocksAddr::Domain(target_host.to_string(), target_port),
        ..Default::default()
    };

    // 3. 通过 VLESS 代理建立一个到目标网站的 TCP 连接隧道
    // A. `handle` 方法返回一个 `ProxyStream`，它是一个枚举，可能是 TCP 或 UDP
    let stream_res = proxy::handle(
        Arc::new(session), // 会话信息
        outbound_handler,  // VLESS 出站处理器
        None, // 不需要 UDP 支持
    ).await;

    let mut stream = match stream_res {
        Ok(ProxyStream::Tcp(tcp_stream)) => {
            println!("\n✅ 成功通过 VLESS 代理建立 TCP 隧道！");
            tcp_stream
        }
        Err(e) => {
            return Err(anyhow!("建立 VLESS 连接失败: {}", e));
        }
        _ => {
            return Err(anyhow!("预期外的流类型"));
        }
    };

    // 4. 在建立的 TCP 隧道上发送 HTTP GET 请求
    println!("正在通过隧道发送 HTTP GET 请求...");
    let http_request = format!(
        "GET /ip HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        target_host
    );

    stream.write_all(http_request.as_bytes()).await?;

    // 5. 读取并打印 HTTP 响应
    let mut response_buffer = Vec::new();
    stream.read_to_end(&mut response_buffer).await?;

    println!("✅ 成功收到响应！");
    println!("-------------------- 响应原文 --------------------");
    println!("{}", String::from_utf8_lossy(&response_buffer));
    println!("-------------------------------------------------");


    Ok(())
}

/// 解析 VLESS URI 并创建 `leaf` 库所需的出站处理器 (OutboundHandler)
fn parse_vless_uri_and_create_handler(uri: &str) -> Result<Arc<dyn OutboundHandler>> {
    let parsed_uri = Url::parse(uri)?;

    // 提取核心信息
    let uuid = parsed_uri.username();
    let host = parsed_uri.host_str().ok_or_else(|| anyhow!("链接中缺少主机地址"))?;
    let port = parsed_uri.port().ok_or_else(|| anyhow!("链接中缺少端口"))?;

    // 将查询参数解析为 HashMap
    let query_params: HashMap<_, _> = parsed_uri.query_pairs().into_owned().collect();

    // 提取关键参数
    let sni = query_params.get("sni").ok_or_else(|| anyhow!("链接中缺少 'sni' 参数"))?;
    let flow = query_params.get("flow").ok_or_else(|| anyhow!("链接中缺少 'flow' 参数"))?;
    let security = query_params.get("security").ok_or_else(|| anyhow!("链接中缺少 'security' 参数"))?;
    let fp = query_params.get("fp").ok_or_else(|| anyhow!("链接中缺少 'fp' 参数"))?;

    // 校验安全设置
    if security != "tls" {
        return Err(anyhow!("仅支持 'security=tls'"));
    }

    // 使用 leaf::outbound::vless::TlsSettings 来构建 TLS 配置
    let tls_settings = leaf::outbound::vless::TlsSettings {
        server_name: sni.clone(),
        fingerprint: fp.clone(),
        alpn: None, // 通常不需要指定
    };

    // 使用 leaf::outbound::vless::Settings 来构建 VLESS 客户端配置
    let vless_settings = leaf::outbound::vless::Settings {
        name: "vless-out".to_string(), // 自定义名称
        server: host.to_string(),
        port,
        uuid: uuid.to_string(),
        flow: flow.clone(),
        tls: Some(tls_settings),
        // 其他传输方式（如 ws）在此处配置，但我们根据链接是 TCP 直连
        transport: None,
    };

    // 根据配置创建 VLESS 客户端实例
    let client = leaf::outbound::vless::Handler::new(vless_settings)?;
    Ok(Arc::new(client))
}

