use base64::{Engine, engine::general_purpose};
use hyper::{Request, body::Incoming, header::PROXY_AUTHORIZATION};

pub fn is_authorized(req: &Request<Incoming>, user: &str, password: &str) -> bool {
  if user.is_empty() || password.is_empty() {
    return true;
  }
  match req.headers().get(PROXY_AUTHORIZATION) {
    Some(header) => {
      if let Ok(header) = header.to_str()
        && let Some(credentials) = header.strip_prefix("Basic ")
        && let Ok(decoded) = general_purpose::STANDARD.decode(credentials)
        && let Ok(decoded_str) = String::from_utf8(decoded)
      {
        let mut parts = decoded_str.splitn(2, ':');
        if let (Some(u), Some(p)) = (parts.next(), parts.next()) {
          return u == user && p == password;
        }
      }
      false
    }
    None => false,
  }
}
