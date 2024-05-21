use std::{collections::BTreeMap, env, ops::Deref, path::PathBuf, str::FromStr};

use anyhow::Result;
pub use async_lazy::Lazy;
pub use fred::{
  self,
  interfaces::ClientLike,
  prelude::{ReconnectPolicy, RedisClient, RedisConfig, ServerConfig},
};
pub use paste::paste;
pub use tracing;
pub use trt::TRT;

pub struct Server;

impl Server {
  pub fn unix_sock(path: impl Into<PathBuf>) -> ServerConfig {
    ServerConfig::Unix { path: path.into() }
  }
  pub fn cluster(hosts: Vec<fred::types::Server>) -> ServerConfig {
    ServerConfig::Clustered {
      hosts,
      policy: Default::default(),
    }
  }

  pub fn sentinel(
    service_name: impl Into<String>,
    hosts: Vec<fred::types::Server>,
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
  pub fn centralized(server: fred::types::Server) -> ServerConfig {
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
  SENTINEL_USER
);

pub struct Wrap(pub &'static Lazy<RedisClient>);

impl Deref for Wrap {
  type Target = RedisClient;
  fn deref(&self) -> &Self::Target {
    self.0.get().unwrap()
  }
}

#[macro_export]
macro_rules! conn {
  ($var:ident = $prefix:ident) => {
    $crate::paste! {
        pub static [<__ $var>]: $crate::Lazy<$crate::RedisClient> = $crate::Lazy::const_new(|| {
          Box::pin(async move {
            let prefix = stringify!($prefix);
            loop {
            match $crate::conn(prefix).await {
              Ok(r)=>return r,
              Err(err)=>{
                eprintln!("❌ Connection Redis {prefix} : {}", err);
                std::process::exit(1);
              }
            }
          }})
        });

        #[static_init::dynamic]
        pub static $var:$crate::Wrap = $crate::Wrap(&[<__ $var>]);

        #[static_init::constructor(0)]
        extern "C" fn [<init_ $prefix:lower>]() {
            $crate::TRT.block_on(async move {
                use std::future::IntoFuture;
                [<__ $var>].into_future().await;
            });
        }
    }
  };
}

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

pub fn server_li(host_port: impl AsRef<str>, default_port: u16) -> Vec<fred::types::Server> {
  use fred::types::Server;
  host_port
    .as_ref()
    .split(' ')
    .map(|i| {
      if let Some(p) = i.rfind(':') {
        let host = i[..p].to_owned();
        if i.len() > p {
          Server::new(host, i[p + 1..].parse().unwrap())
        } else {
          Server::new(host.to_owned(), default_port)
        }
      } else {
        Server::new(i.to_owned(), default_port)
      }
    })
    .collect()
}

pub async fn conn(prefix: impl AsRef<str>) -> Result<RedisClient> {
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
    .unwrap_or_else(|| unreachable!("NEED ENV {prefix}{}", NODE));

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

  connect(&server, user, password, database).await
}

pub async fn connect(
  server: &ServerConfig,
  username: Option<String>,
  password: Option<String>,
  database: Option<u8>,
) -> Result<RedisClient> {
  let mut conf = RedisConfig {
    version: fred::types::RespVersion::RESP3,
    ..Default::default()
  };
  conf.server = server.clone();
  conf.username = username;
  conf.password = password;
  conf.database = database;
  /*
  https://docs.rs/fred/6.2.1/fred/types/enum.ReconnectPolicy.html#method.new_constant
  */
  let policy = ReconnectPolicy::new_constant(6, 1);
  let client = RedisClient::new(conf, None, None, Some(policy));
  client.connect();
  client.wait_for_connect().await?;
  Ok(client)
}
