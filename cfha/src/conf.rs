use std::{
  collections::HashMap,
  net::{Ipv4Addr, Ipv6Addr},
};

use aok::Result;

pub struct Host {
  pub host: String,
  pub ipv4: Vec<(String, Ipv4Addr)>,
  pub ipv6: Vec<(String, Ipv6Addr)>,
}

pub fn yml(conf: &str) -> Result<()> {
  let conf: HashMap<String, HashMap<String, Vec<String>>> = serde_yaml::from_str(conf)?;
  println!("{:?}", conf);
  // for (k, v) in conf {}
  Ok(())
}
