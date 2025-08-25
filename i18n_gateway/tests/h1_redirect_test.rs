use std::net::SocketAddr;

mod i18n_gateway_srv;
mod util;

use i18n_gateway_srv::TEST_HOST;

#[tokio::test]
async fn test_h1_redirect_and_not_found() -> anyhow::Result<()> {
  let path = "/test-path";
  let h1_addr: SocketAddr = i18n_gateway_srv::H1_ADDR.parse()?;

  // --- 测试场景 1: 访问已配置的域名,应该重定向到 h2 ---
  let res = util::get(&format!("http://{TEST_HOST}{path}"), h1_addr).await?;

  // 验证状态码是否为 301 (永久重定向)
  assert_eq!(res.status(), reqwest::StatusCode::MOVED_PERMANENTLY);

  // 验证 Location 头是否指向 h2 服务地址
  let location = res.headers().get("Location").unwrap().to_str()?;
  let expected_location = format!("https://{TEST_HOST}{path}");
  assert_eq!(location, expected_location);

  let url = format!("http://{TEST_HOST}{path}");
  println!("重定向测试通过: {url} -> {location}");

  // --- 测试场景 2: 访问未配置的域名,应该返回 404 ---
  let unknown_host = "unknown.com";
  let res_404 = util::get(&format!("http://{unknown_host}{path}"), h1_addr).await?;

  // 验证状态码是否为 404 (未找到)
  assert_eq!(res_404.status(), reqwest::StatusCode::NOT_FOUND);
  println!("404 未找到测试通过: Host={unknown_host}");

  Ok(())
}
