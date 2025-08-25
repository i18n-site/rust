use std::{
  collections::{HashMap, HashSet},
  net::SocketAddr,
  sync::Arc,
};

use dashmap::DashMap;
use faststr::FastStr;

#[derive(Debug, Clone)]
pub struct SiteConf {
  pub upstream: Arc<Upstream>,
  pub use_wildcast_cert: bool,
}

impl SiteConf {
  pub fn new(upstream: Arc<Upstream>, use_wildcast_cert: bool) -> Self {
    Self {
      upstream,
      use_wildcast_cert,
    }
  }
}

#[derive(PartialEq,Eq,Debug)]
pub enum Protocol {
  H1,
  H2,
  H3,
}

#[derive(Debug)]
pub struct Upstream {
  pub addr_li: Box<[SocketAddr]>,
  pub connect_timeout_sec: u64,
  pub request_timeout_sec: u64,
  pub max_retry: usize,
  pub protocol: Protocol,
}

#[derive(Debug)]
pub struct UpstreamSiteSet {
  pub upstream: Arc<Upstream>,
  pub host_set: HashSet<FastStr>,
}

#[derive(Debug, Default)]
pub struct Route {
  pub host_conf: DashMap<FastStr, SiteConf>,
  pub upstream_site: HashMap<FastStr, UpstreamSiteSet>,
}

impl Route {
  /// 设置 域名 到 服务器 的映射
  pub fn set(
    &mut self,
    host: impl Into<FastStr>,
    use_wildcast_cert: bool,
    upstream_name: impl Into<FastStr>,
  ) -> &mut Self {
    let upstream_name = upstream_name.into();
    let host = host.into();
    if let Some(t) = self.upstream_site.get_mut(&upstream_name) {
      t.host_set.insert(host.clone());
      self
        .host_conf
        .insert(host, SiteConf::new(t.upstream.clone(), use_wildcast_cert));
    }
    self
  }
}
