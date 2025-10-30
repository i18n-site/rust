use std::collections::HashMap;

use serde::Deserialize;
use url::Url;

use crate::error::HysteriaError;

#[derive(Debug, Deserialize)]
pub struct Config {
  pub auth: String,
  pub server_addr: String,
  pub server_name: String,
  pub insecure: bool,
  pub port_hopping_range: Option<(u16, u16)>,
}

impl Config {
  pub fn from_url(url_str: &str) -> Result<Self, HysteriaError> {
    let url = Url::parse(&url_str.replace("hysteria2://", "http://"))?;

    let host = url
      .host_str()
      .ok_or_else(|| HysteriaError::UrlParseError(url::ParseError::EmptyHost))?;
    let port = url
      .port()
      .ok_or_else(|| HysteriaError::UrlParseError(url::ParseError::InvalidPort))?;

    // Collect all query parameters at once to avoid multiple iterations.
    // 一次性收集所有查询参数，避免多次迭代。
    let query_params: HashMap<String, String> = url.query_pairs().into_owned().collect();

    let server_name = query_params
      .get("sni")
      .cloned()
      .unwrap_or_else(|| host.to_string());

    let insecure = query_params.get("insecure").is_some_and(|v| v == "1");

    let port_hopping_range = query_params
      .get("mport")
      .and_then(|v| Self::parse_port_range(v));

    Ok(Config {
      auth: url.username().to_string(),
      server_addr: format!("{}:{}", host, port),
      server_name,
      insecure,
      port_hopping_range,
    })
  }

  /// Parse port range, which can be a single port or "start-end".
  /// 解析端口范围，可以是单个端口或 "start-end"。
  fn parse_port_range(range_str: &str) -> Option<(u16, u16)> {
    if let Some((start_str, end_str)) = range_str.split_once('-') {
      let start = start_str.trim().parse().ok()?;
      let end = end_str.trim().parse().ok()?;
      if start > end {
        None
      } else {
        Some((start, end))
      }
    } else {
      let port = range_str.trim().parse().ok()?;
      Some((port, port))
    }
  }
}
