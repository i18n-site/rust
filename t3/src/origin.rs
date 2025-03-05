use axum::http::header::{HeaderMap, ORIGIN};
use re::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("http headers miss origin")]
  HeaderMissOrigin,
}

pub fn origin(header: &HeaderMap) -> Result<&str> {
  if let Some(o) = header.get(ORIGIN) {
    return Ok(xtld::url_host_port(o.to_str()?));
  }
  Err(Error::HeaderMissOrigin.into())
}

pub fn origin_tld(header: &HeaderMap) -> Result<&str> {
  Ok(xtld::host_port_tld(origin(header)?))
}
