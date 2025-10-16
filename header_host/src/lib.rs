#[derive(Debug)]
pub struct HeaderNoHost;

impl std::fmt::Display for HeaderNoHost {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "no host")
  }
}

impl std::error::Error for HeaderNoHost {}

pub fn host(headers: &http::HeaderMap) -> Result<&str, HeaderNoHost> {
  let host = if let Some(host) = headers.get("x-forwarded-host") {
    host
  } else if let Some(host) = headers.get("host") {
    host
  } else {
    Err(HeaderNoHost)?;
    unreachable!();
  }
  .to_str();
  match host {
    Ok(host) => Ok(host),
    Err(_) => Err(HeaderNoHost),
  }
}

#[cfg(feature = "tld")]
pub fn tld(headers: &http::HeaderMap) -> Result<&str, HeaderNoHost> {
  Ok(xtld::tld(host(headers)?))
}
