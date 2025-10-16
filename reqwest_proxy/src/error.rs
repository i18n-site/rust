#[derive(thiserror::Error, Debug)]
pub enum Error {
  /// IO error
  /// IO 错误
  #[error("I/O error: {0}")]
  Io(#[from] std::io::Error),

  /// Shadowsocks URL parse error
  /// Shadowsocks URL 解析错误
  #[cfg(feature = "shadowsocks")]
  #[error("Invalid Shadowsocks URL: {0}")]
  InvalidSsUrl(#[from] shadowsocks::config::UrlParseError),

  /// Hysteria2 error
  /// Hysteria2 错误
  #[cfg(feature = "hysteria2")]
  #[error("Hysteria2 error: {0}")]
  Hysteria2(#[from] hysteria2::HysteriaError),

  /// Reqwest request conversion error
  /// reqwest 请求转换错误
  #[error("reqwest request conversion error: {0}")]
  ReqwestRequestConversion(reqwest::Error),

  /// Hyper client error
  /// hyper 客户端错误
  #[error("hyper client error: {0}")]
  HyperClient(hyper_util::client::legacy::Error),

  /// Hyper body conversion error
  /// hyper body 转换错误
  #[error("hyper body error: {0}")]
  HyperBody(hyper::Error),

  /// Unsupported Protocol error
  ///不支持的协议错误
  #[error("Unsupported Protocol: {0}")]
  UnsupportedProtocol(String),
}
