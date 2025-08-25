use std::sync::Arc;

use crate::{
  cert_loader::{CertLoader, CertStrDb},
  conf::Conf,
  error::Error,
};

mod h1;
mod h2;
mod h3;
mod h3_conn;
mod proxy;
mod util;
use proxy::proxy;

/// 启动所有服务
pub async fn run<D: CertStrDb + std::fmt::Debug + Send + Sync + 'static>(
  conf: Conf,
  cert_db: D,
) -> Result<(), Error> {
  let cert_loader = CertLoader::new(cert_db);

  let route = Arc::new(conf.route);

  // 分别启动 h1, h2, h3 服务
  // tokio::try_join! 会在一个任务失败时取消其他任务
  tokio::try_join!(
    h1::run(conf.h1, route.clone()),
    h2::run(conf.h2, route.clone(), cert_loader.clone()),
    h3::run(conf.h3, route.clone(), cert_loader.clone())
  )?;

  Ok(())
}
