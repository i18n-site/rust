use std::str::FromStr;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

pub fn header_map<HN, HV>(headers: impl IntoIterator<Item = (HN, HV)>) -> HeaderMap
where
  HN: AsRef<str>,
  HV: AsRef<str>,
{
  let mut header_map = HeaderMap::new();

  for (key, value) in headers {
    if let Ok(key) = xerr::ok!(HeaderName::from_str(key.as_ref())) {
      if let Ok(value) = xerr::ok!(HeaderValue::from_str(value.as_ref())) {
        header_map.insert(key, value);
      }
    }
  }

  header_map
}
