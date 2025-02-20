use axum::http::header::{HeaderMap, HOST};
use re::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("http headers miss host")]
  HeaderMissHost,
}

pub fn host(header: &HeaderMap) -> Result<String> {
  if let Some(o) = header.get(HOST) {
    return Ok(xtld::url_host_port(o.to_str()?));
  }
  Err(Error::HeaderMissHost)?
}

pub fn host_tld(header: &HeaderMap) -> Result<String> {
  Ok(xtld::host_port_tld(host(header)?))
}
