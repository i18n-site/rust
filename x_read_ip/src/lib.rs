#![cfg_attr(docsrs, feature(doc_cfg))]

use std::{borrow::Borrow, net::IpAddr};

use http::HeaderMap;

pub fn get(headers: impl Borrow<HeaderMap>) -> Vec<u8> {
  let header_candidates = ["x-forwarded-for", "x-real-ip", "cf-connecting-ip"];

  for header_name in header_candidates {
    if let Some(header_value) = headers.borrow().get(header_name)
      && let Ok(raw_str) = header_value.to_str()
    {
      let ip_str = raw_str.split(',').next().unwrap_or("").trim();

      if let Ok(ip) = ip_str.parse::<IpAddr>() {
        return match ip {
          IpAddr::V4(ipv4) => ipv4.octets().to_vec(),
          IpAddr::V6(ipv6) => ipv6.octets().to_vec(),
        };
      }
    }
  }

  vec![]
}
