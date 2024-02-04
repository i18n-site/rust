use std::{collections::HashMap, net::IpAddr};

use dashmap::DashMap;
use m::{id_row, id_v_str, q};
use mysql_async::Result;
use xstr::Join;

use crate::db::Kind;

#[derive(Debug, Clone)]
pub struct Ip {
  pub ip: IpAddr,
  pub name: String,
}

pub async fn id_ip(table: &str, id_set: impl IntoIterator<Item = u64>) -> Result<HashMap<u64, Ip>> {
  let ids = id_set.join(",");
  let mut m = HashMap::new();
  let li: Vec<(u64, Vec<u8>, String)> =
    q!(format!("SELECT id,v,name FROM {table} WHERE id IN({ids})"));
  for (id, bin, name) in li {
    let len = bin.len();
    let ip = match len {
      4 => IpAddr::V4(TryInto::<[u8; 4]>::try_into(bin).unwrap().into()),
      16 => IpAddr::V6(TryInto::<[u8; 16]>::try_into(bin).unwrap().into()),
      _ => {
        tracing::error!("ip.id={} UNKNOWN IP {:?}", id, bin);
        continue;
      }
    };
    m.insert(id, Ip { ip, name });
  }

  Ok(m)
}

macro_rules! cache {
  ($fn:ident, $cls:ident, $val_type:ty, $id_x:ident) => {
    #[static_init::dynamic]
    pub static $cls: DashMap<u64, $val_type> = DashMap::new();

    pub async fn $fn(set: impl IntoIterator<Item = u64>) -> Result<HashMap<u64, $val_type>> {
      let mut m = HashMap::new();

      let mut not_exist = Vec::new();
      for i in set {
        if let Some(k) = $cls.get(&i) {
          m.insert(i, k.clone());
        } else {
          not_exist.push(i);
        }
      }

      if !not_exist.is_empty() {
        let map: HashMap<u64, $val_type> = $id_x(stringify!($fn), not_exist).await?;
        for i in map {
          $cls.insert(i.0, i.1.clone());
          m.insert(i.0, i.1);
        }
      }

      Ok(m)
    }
  };
}

cache!(kind, KIND, Kind, id_row);
cache!(host, HOST, String, id_v_str);
cache!(arg, ARG, String, id_v_str);
cache!(ip, IP, Ip, id_ip);
