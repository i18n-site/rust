use thiserror::Error;

/// SsConnector 的主要错误类型
#[derive(Error, Debug)]
pub enum SsConnectorError {
  /// 当解析 Shadowsocks URL 失败时返回
  #[error("无效的 Shadowsocks URL: {0}")]
  InvalidUrl(#[from] shadowsocks::config::UrlParseError),

  /// reqwest 请求转换错误
  #[error("reqwest 请求转换错误: {0}")]
  ReqwestRequestConversion(reqwest::Error),

  /// hyper 客户端错误
  #[error("hyper 客户端错误: {0}")]
  HyperClient(hyper_util::client::legacy::Error),

  /// hyper body 转换错误
  #[error("hyper body 转换错误: {0}")]
  HyperBody(hyper::Error),
}
