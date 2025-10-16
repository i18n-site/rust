use hyper::Uri;

mod error;
pub use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub fn parse(uri: impl TryInto<Uri> + std::fmt::Debug + Clone) -> Result<(String, u16)> {
  let uri_object = uri
    .clone()
    .try_into()
    .map_err(|_| Error::InvalidUri(format!("{uri:?}")))?;
  let host = uri_object
    .host()
    .ok_or_else(|| Error::MissingHost(format!("{uri:?}")))?;
  let port = uri_object
    .port_u16()
    .unwrap_or_else(|| match uri_object.scheme_str() {
      Some("https") => 443,
      _ => 80,
    });
  Ok((host.to_string(), port))
}
