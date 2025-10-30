#![cfg_attr(docsrs, feature(doc_cfg))]
#![feature(macro_metavar_expr)]

use std::{collections::BTreeMap, env, path::PathBuf, str::FromStr};

use aok::Result;
use err_exit::err_exit;
pub use fred;
use fred::{
  interfaces::ClientLike,
  prelude::{Client, Config, ReconnectPolicy, Server as FredServer, ServerConfig},
};

#[cfg(feature = "macro")]
mod r#macro;

#[cfg(feature = "macro")]
mod macro_pub_use {
  pub use log;
  pub use static_;
}

#[cfg(feature = "macro")]
pub use macro_pub_use::*;

#[cfg(feature = "r")]
mod r;

#[cfg(feature = "r")]
pub use r::R;

pub struct Server;

impl Server {
  pub fn unix_sock(path: impl Into<PathBuf>) -> ServerConfig {
    ServerConfig::Unix { path: path.into() }
  }
  pub fn cluster(hosts: Vec<FredServer>) -> ServerConfig {
    ServerConfig::Clustered {
      hosts,
      policy: Default::default(),
    }
  }

  pub fn sentinel(
    service_name: impl Into<String>,
    hosts: Vec<FredServer>,
    username: Option<String>,
    password: Option<String>,
  ) -> ServerConfig {
    ServerConfig::Sentinel {
      service_name: service_name.into(),
      hosts,
      username: Some(username.unwrap_or_else(|| "default".into())),
      password,
    }
  }
  pub fn centralized(server: FredServer) -> ServerConfig {
    ServerConfig::Centralized { server }
  }
}

macro_rules! env {
  ($($name:ident),*)=>{
    $(
      const $name: &str = stringify!($name);
    )*
    const REDIS_ENV_LI: &[&str] = &[$($name),*];
  }
}

env!(
  USER,
  NODE,
  PASSWORD,
  DB,
  SENTINEL_NAME,
  SENTINEL_PASSWORD,
  SENTINEL_USER,
  RESP
);

fn get(u: Option<&String>) -> Option<String> {
  if let Some(u) = u {
    if u.is_empty() {
      None
    } else {
      Some(u.to_owned())
    }
  } else {
    None
  }
}

pub fn server_li(host_port: impl AsRef<str>, default_port: u16) -> Vec<FredServer> {
  host_port
    .as_ref()
    .split(' ')
    .map(|i| {
      if let Some(p) = i.rfind(':') {
        let host = i[..p].to_owned();
        if i.len() > p {
          FredServer::new(host, i[p + 1..].parse().unwrap())
        } else {
          FredServer::new(host.to_owned(), default_port)
        }
      } else {
        FredServer::new(i.to_owned(), default_port)
      }
    })
    .collect()
}

pub async fn conn(prefix: impl AsRef<str>) -> Result<Client> {
  let prefix = prefix.as_ref().to_owned() + "_";

  let mut map = BTreeMap::new();

  for (key, value) in env::vars() {
    if key.starts_with(&prefix) {
      let key = &key[prefix.len()..];

      if REDIS_ENV_LI.contains(&key) {
        map.insert(key.to_owned(), value.trim().to_owned());
      }
    }
  }
  let host_port = map
    .get(NODE)
    .unwrap_or_else(|| err_exit!("xkv : miss env {prefix}{}", NODE));

  let server = if let Some(sentinel_name) = map.get(SENTINEL_NAME).cloned() {
    Server::sentinel(
      sentinel_name,
      server_li(host_port, 26379),
      map.get(SENTINEL_USER).cloned(),
      map.get(SENTINEL_PASSWORD).cloned(),
    )
  } else if host_port.starts_with('/') {
    Server::unix_sock(host_port)
  } else {
    let mut host_port = server_li(host_port, 6379);

    if host_port.len() == 1 {
      Server::centralized(host_port.pop().unwrap())
    } else {
      Server::cluster(host_port)
    }
  };

  let database = get(map.get(DB)).map(|s| u8::from_str(&s).unwrap());
  let user = get(map.get(USER));
  let password = get(map.get(PASSWORD));

  connect(
    &server,
    user,
    password,
    database,
    map.get(RESP).map(|i| i.as_str()),
  )
  .await
}

pub async fn connect(
  server: &ServerConfig,
  username: Option<String>,
  password: Option<String>,
  database: Option<u8>,
  resp: Option<&str>,
) -> Result<Client> {
  let mut conf = Config {
    version: if resp == Some("2") {
      fred::types::RespVersion::RESP2
    } else {
      fred::types::RespVersion::RESP3
    },
    ..Default::default()
  };
  conf.server = server.clone();
  conf.username = username;
  conf.password = password;
  conf.database = database;
  /*
  https://docs.rs/fred/6.2.1/fred/types/enum.ReconnectPolicy.html#method.new_constant
  */
  let policy = ReconnectPolicy::new_linear(u32::MAX, 8, 1);
  let client = Client::new(conf, None, None, Some(policy));
  // client.connect();
  // client.wait_for_connect().await?;
  client.init().await?;
  Ok(client)
}
