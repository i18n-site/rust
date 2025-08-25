use std::net::SocketAddr;

use crate::route::Route;

/// 全局配置
#[derive(Debug)]
pub struct Conf {
  pub h1: SocketAddr,
  pub h2: SocketAddr,
  pub h3: SocketAddr,
  pub route: Route,
}
