pub mod ping;

use std::{net::IpAddr, time::Duration};

mod r#macro;

pub use hickory_proto;
use hickory_proto::rr::{record_type::RecordType, RData, Record};
pub use hickory_resolver;
use hickory_resolver::{
  config::{NameServerConfigGroup, ResolverConfig, ResolverOpts},
  error::ResolveError,
  lookup::Lookup,
  IntoName, TokioAsyncResolver,
};

pub fn resolve_options() -> ResolverOpts {
  let mut opt = ResolverOpts::default();
  opt.attempts = 1;
  opt.num_concurrent_reqs = 3;
  opt.timeout = Duration::from_secs(6);
  opt
}

pub fn resolver(ip: &[IpAddr]) -> TokioAsyncResolver {
  TokioAsyncResolver::tokio(
    ResolverConfig::from_parts(
      None,
      vec![],
      NameServerConfigGroup::from_ips_clear(
        // https://www.chengxulvtu.net/public-dns/
        ip, 53, true,
      ),
    ),
    resolve_options(),
  )
}

#[static_init::dynamic]
pub static RESOLVER_IPV4: TokioAsyncResolver = resolver(&ip!(
    208,67,220,220;
    208,67,222,222;
    77,88,8,8;
    77,88,8,1;
    114,114,114,114;
    114,114,115,115;
    119,28,28,28;
    119,29,29,29;
    149,112,112,112;
    180,76,76,76;
    1,0,0,1;
    1,1,1,1;
    223,5,5,5;
    223,6,6,6;
    8,8,4,4;
    9,9,9,9;
    8,8,8,8;
));

#[static_init::dynamic]
pub static RESOLVER_IPV6: TokioAsyncResolver = resolver(&ip!(
0x2606, 0x4700, 0x4700, 0, 0, 0, 0, 0x64;
0x2400, 0x3200, 0, 0, 0, 0, 0, 0x0001;
0x2400, 0xda00, 0, 0, 0, 0, 0, 0x6666;
0x2001, 0x4860, 0x4860, 0, 0, 0, 0, 0x8888;
));

#[static_init::dynamic]
pub static RESOLVER_TLS: TokioAsyncResolver = {
  let mut conf = ResolverConfig::default();
  for dns in [
    // NameServerConfigGroup::from_ips_https(
    //   &[
    //     IpAddr::V4([223, 6, 6, 6].into()),
    //     IpAddr::V4([223, 5, 5, 5].into()),
    //   ],
    //   443,
    //   "dns.alidns.com".into(),
    //   true,
    // ),
    // NameServerConfigGroup::from_ips_https(
    //   &[
    //     IpAddr::V4([120, 53, 53, 53].into()),
    //     IpAddr::V4([1, 12, 12, 12].into()),
    //   ],
    //   443,
    //   "doh.pub".into(),
    //   true,
    // ),
    NameServerConfigGroup::google_https(),
    NameServerConfigGroup::cloudflare_https(),
    NameServerConfigGroup::quad9_https(),
  ] {
    for i in dns.iter() {
      conf.add_name_server(i.clone());
    }
  }
  TokioAsyncResolver::tokio(conf, resolve_options())
};

pub async fn lookup<N: IntoName>(
  resolver: &TokioAsyncResolver,
  name: N,
  record_type: RecordType,
) -> Result<Lookup, ResolveError> {
  let name = name.into_name()?;
  match resolver.lookup(name.clone(), record_type).await {
    Ok(r) => Ok(r),
    Err(err) => {
      tracing::warn!("{name} DNS ERROR: {}", err);
      RESOLVER_TLS.lookup(name, record_type).await
    }
  }
}

pub fn records_ip<'a>(records: impl IntoIterator<Item = &'a Record>) -> Vec<std::net::IpAddr> {
  records
    .into_iter()
    .filter_map(|r| match r.data() {
      Some(RData::A(ip)) => Some(std::net::IpAddr::V4(**ip)),
      Some(RData::AAAA(ip)) => Some(std::net::IpAddr::V6(**ip)),
      _ => None,
    })
    .collect()
}

macro_rules! ip_li {
  ($resolver:expr,$host:ident,$type:ident, $ver:ident) => {{
    let li = lookup($resolver, $host, RecordType::$type).await?;
    Ok(records_ip(li.records()))
  }};
}

#[allow(non_snake_case)]
pub async fn A<N: IntoName>(name: N) -> Result<Vec<std::net::IpAddr>, ResolveError> {
  ip_li!(&*RESOLVER_IPV4, name, A, V4)
}

#[allow(non_snake_case)]
pub async fn AAAA<N: IntoName>(name: N) -> Result<Vec<std::net::IpAddr>, ResolveError> {
  ip_li!(&*RESOLVER_IPV6, name, AAAA, V6)
}

static mut USE_IPV6: bool = true;

pub async fn ip<N: IntoName>(name: N) -> Result<Vec<std::net::IpAddr>, ResolveError> {
  let name = name.into_name()?;
  if unsafe { USE_IPV6 } {
    if let Ok(r) = RESOLVER_IPV6.lookup(name.clone(), RecordType::AAAA).await {
      let r = records_ip(r.records());
      if !r.is_empty() {
        return Ok(r);
      }
    } else {
      unsafe {
        USE_IPV6 = false;
      }
    }
  }

  match A(name).await {
    Ok(r) => Ok(r),
    Err(err) => {
      unsafe {
        USE_IPV6 = true;
      }
      Err(err)
    }
  }
}
