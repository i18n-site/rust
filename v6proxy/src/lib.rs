use std::{collections::HashMap, time::Duration};

use aok::Result;
use reqwest::{Client, Proxy};

genv::s!(
  IPV6_PROXY_USER,
  IPV6_PROXY_PASSWD,
  IPV6_PROXY_PORT:u16,
  IPV6_PROXY_IP_LI
);

const TIMEOUT: Duration = Duration::from_secs(60);

pub fn proxy(proxy: Proxy) -> reqwest::Client {
  Client::builder()
        .proxy(proxy)
        .zstd(true)
        // .http3_prior_knowledge()
        .timeout(TIMEOUT)
        .danger_accept_invalid_certs(true)
        .connect_timeout(TIMEOUT).build().unwrap()
}

pub struct Host {
  pub name: String,
  pub client: reqwest::Client,
}

pub fn from_env() -> Result<Vec<Host>> {
  let url = format!("http://{}:{}@", *IPV6_PROXY_USER, *IPV6_PROXY_PASSWD,);
  let port: u16 = *IPV6_PROXY_PORT;

  let name_ip: HashMap<String, String> = sonic_rs::from_str(&IPV6_PROXY_IP_LI)?;

  let li = name_ip
    .into_iter()
    .map(|(name, ip)| {
      Ok(Host {
        name,
        client: proxy(reqwest::Proxy::https(format!("{url}{ip}:{port}"))?),
      })
    })
    .collect::<Result<_, aok::Error>>()?;

  Ok(li)
}

#[static_init::dynamic]
pub static HOST_LI: Vec<Host> = from_env().unwrap();
